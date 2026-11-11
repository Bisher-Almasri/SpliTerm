// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::Mutex;
use std::{env, sync::Arc};
use std::io::{Read, Write};
use std::thread;

mod config;

use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use tauri::{Manager, State};
use crate::config::AppConfig;

struct PtyState {
    master: Arc<Mutex<Option<Box<dyn MasterPty + Send>>>>,
    writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    // reader: Arc<Mutex<Option<Box<dyn Read + Send>>>>
}

struct PtyStates {
    states: Mutex<Vec<PtyState>>
}

struct ConfigState(Mutex<AppConfig>);


#[tauri::command]
fn init(app: tauri::AppHandle,rows: u16, cols: u16, state: tauri::State<'_, PtyStates>) -> Result<usize, String> {
    let pty_system = native_pty_system();

    let pair = pty_system.openpty(PtySize {
        rows: rows,
        cols: cols,
        pixel_width: 0,
        pixel_height: 0
    }).map_err(|e| e.to_string())?;

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
                    let _ = tauri::Emitter::emit(&app, &format!("pty-output-{id}"), error.to_string());
                    break;
                }
            }
        }

        let _ = child.wait();
    });

    Ok(id)
}

#[tauri::command]
fn write_to_pty(data: String, index: usize, state: tauri::State<'_, PtyStates>) -> Result<(), String> {
    let state = state.states.lock().unwrap();
    let pty = state.get(index).ok_or("Invalid PTY index")?;

    let mut writer_guard = pty.writer.lock().unwrap();
    let writer = writer_guard.as_mut().ok_or("PTY not initialized")?;


    writer.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn resize_terminal(index: usize, rows: u16, cols: u16, state: tauri::State<'_, PtyStates>) {
    let state = state.states.lock().unwrap();
    let pty = state.get(index).ok_or("Invalid PTY index").unwrap();

    let mut master_gaurd = pty.master.lock().unwrap();

    let master = master_gaurd.as_mut().unwrap();

    master.resize(PtySize { rows: rows, cols: cols, pixel_width: 0, pixel_height: 0}).unwrap();
}

#[tauri::command]
fn get_config(state: State<ConfigState>) -> AppConfig {
    state.0.lock().unwrap().clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(PtyStates {
            states: Mutex::new(Vec::new()),
        })
        .setup(|app| {
            let cfg = AppConfig::load_or_create(app.handle())?;
            app.manage(ConfigState(Mutex::new(cfg)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init, write_to_pty, resize_terminal, get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}