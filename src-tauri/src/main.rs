// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use mini_redis::client;
use mini_redis::Result as RedisResult;
use window_shadows::set_shadow;



async fn say_world() {
    println!("world!");
}

#[tokio::main]
async fn main() -> RedisResult<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let redis_result = client.get("hello").await?;

    let op = say_world();

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
        .run(context)?;

        println!("got value from the server; result={:?}", redis_result);
        println!("Hello");

        op.await;

    Ok(())
}
