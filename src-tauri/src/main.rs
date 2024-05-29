// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::vec;

use prisma::{user::role, PrismaClient};
use tauri::Manager;
use window_shadows::set_shadow;

mod error;
use error::Error;

#[allow(warnings)]
mod prisma;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let context = tauri::generate_context!();
    let _app = tauri::Builder::default()
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
        .run(context)
        .expect("error while running tauri application");

    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Cannot connect to database");

    let new_user = client
        .user()
        .create("test@test.com".into(), "Passord01".into(), vec![])
        .exec()
        .await?;

    println!("{:#?}",new_user);
    Ok(())
}
