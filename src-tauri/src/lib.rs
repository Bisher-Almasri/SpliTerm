// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::Mutex;
use std::{env, sync::Arc};
use std::io::{Read, Write};
use std::thread;

use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};

struct PtyState {
    master: Arc<Mutex<Option<Box<dyn MasterPty + Send>>>>,
    writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    // reader: Arc<Mutex<Option<Box<dyn Read + Send>>>>
}

#[tauri::command]
fn init(app: tauri::AppHandle, state: tauri::State<'_, PtyState>) {
    let pty_system = native_pty_system();

    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0
    }).map_err(|e| e.to_string()).unwrap();

    let cmd = CommandBuilder::new(env::var("SHELL").unwrap_or(String::from("bash")));

    let mut child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string()).unwrap();

    drop(pair.slave);   
    let writer: Box<dyn Write + Send> = pair.master.take_writer().unwrap(); 
    let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string()).unwrap();
    *state.master.lock().unwrap() = Some(pair.master);
    *state.writer.lock().unwrap() = Some(writer);
    // *state.reader.lock().unwrap() = Some(reader);


    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(size) => {
                    let output = String::from_utf8_lossy(&buffer[..size]).into_owned();
                    let _ = tauri::Emitter::emit(&app, "pty-output", output);
                }
                Err(error) => {
                    let _ = tauri::Emitter::emit(&app, "pty-error", error.to_string());
                    break;
                }
            }
        }

        let _ = child.wait();
    });
}

#[tauri::command]
fn write_to_pty(data: String, state: tauri::State<'_, PtyState>) -> Result<(), String> {
    let mut writer = state.writer.lock().unwrap();
    let writer = writer.as_mut().ok_or("PTY not initialized")?;

    writer.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(PtyState {
            master: Arc::new(Mutex::new(None)),
            writer: Arc::new(Mutex::new(None)),
            // reader: Arc::new(Mutex::new(None)) 
        })
        .invoke_handler(tauri::generate_handler![init, write_to_pty])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
