// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::io::{Read, Write};
use std::sync::Mutex;
use std::thread;
use std::{env, sync::Arc};

mod config;

use crate::config::AppConfig;
use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use tauri::{Manager, State, WebviewUrl, WebviewWindowBuilder};

struct PtyState {
    master: Arc<Mutex<Option<Box<dyn MasterPty + Send>>>>,
    writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    // reader: Arc<Mutex<Option<Box<dyn Read + Send>>>>
}

struct PtyStates {
    states: Mutex<Vec<PtyState>>,
}

struct ConfigState(Mutex<AppConfig>);

#[tauri::command]
fn init(
    app: tauri::AppHandle,
    rows: u16,
    cols: u16,
    state: tauri::State<'_, PtyStates>,
) -> Result<usize, String> {
    let pty_system = native_pty_system();

    let pair = pty_system
        .openpty(PtySize {
            rows: rows,
            cols: cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| e.to_string())?;

    let cmd = CommandBuilder::new(env::var("SHELL").unwrap_or_else(|_| String::from("bash")));

    let mut child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;

    drop(pair.slave);
    let writer: Box<dyn Write + Send> = pair.master.take_writer().map_err(|e| e.to_string())?;
    let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;

    let new_pty = PtyState {
        master: Arc::new(Mutex::new(Some(pair.master))),
        writer: Arc::new(Mutex::new(Some(writer))),
    };

    let mut states = state.states.lock().map_err(|e| e.to_string())?;
    let id = states.len();
    states.push(new_pty);
    // *state.reader.lock().unwrap() = Some(reader);

    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(size) => {
                    let output = String::from_utf8_lossy(&buffer[..size]).into_owned();
                    let _ = tauri::Emitter::emit(&app, &format!("pty-output-{id}"), output);
                }
                Err(error) => {
                    let _ =
                        tauri::Emitter::emit(&app, &format!("pty-output-{id}"), error.to_string());
                    break;
                }
            }
        }

        let _ = child.wait();
    });

    Ok(id)
}

#[tauri::command]
fn write_to_pty(
    data: String,
    index: usize,
    state: tauri::State<'_, PtyStates>,
) -> Result<(), String> {
    let state = state.states.lock().unwrap();
    let pty = state.get(index).ok_or("Invalid PTY index")?;

    let mut writer_guard = pty.writer.lock().unwrap();
    let writer = writer_guard.as_mut().ok_or("PTY not initialized")?;

    writer
        .write_all(data.as_bytes())
        .map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn resize_terminal(index: usize, rows: u16, cols: u16, state: tauri::State<'_, PtyStates>) {
    let state = state.states.lock().unwrap();
    let pty = state.get(index).ok_or("Invalid PTY index").unwrap();

    let mut master_guard = pty.master.lock().unwrap();

    let master = master_guard.as_mut().unwrap();

    master
        .resize(PtySize {
            rows: rows,
            cols: cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();
}

#[tauri::command]
fn get_config(state: State<ConfigState>) -> AppConfig {
    state.0.lock().unwrap().clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let cfg = AppConfig::load_or_create(app.handle())?;

            let bg_hex = cfg.theme.as_ref().and_then(|t| t.background.clone());

            app.manage(ConfigState(Mutex::new(cfg)));

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("SpliTerm")
                .transparent(true)
                .decorations(false)
                .inner_size(800.0, 600.0);

            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(tauri::TitleBarStyle::Transparent);

            let window = win_builder.build().unwrap();

            #[cfg(target_os = "macos")]
            {
                use objc2_app_kit::NSWindow;

                let ns_window_ptr = window.ns_window().unwrap() as *mut NSWindow;
                let ns_window = unsafe { &*ns_window_ptr };

                let bg_color = bg_hex.as_deref().and_then(hex_to_ns_color);
                ns_window.setBackgroundColor(bg_color.as_deref());
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(PtyStates {
            states: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            init,
            write_to_pty,
            resize_terminal,
            get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "macos")]
fn hex_to_ns_color(hex: &str) -> Option<objc2::rc::Retained<objc2_app_kit::NSColor>> {
    let hex = hex.trim().trim_start_matches('#');

    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        4 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            let a = u8::from_str_radix(&hex[3..4], 16).ok()? * 17;
            (r, g, b, a)
        }
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            (r, g, b, 255)
        }
        _ => return None,
    };

    Some(objc2_app_kit::NSColor::colorWithSRGBRed_green_blue_alpha(
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0,
        a as f64 / 255.0,
    ))
}
