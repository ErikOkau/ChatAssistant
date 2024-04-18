// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_shadows::set_shadow;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {

            let window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "windows"))]
            set_shadow(&window, true).unwrap();

            Ok(())
        })
        .on_window_event(|event| match event.event() {
          tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
          }
          _ => {}
        })

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
