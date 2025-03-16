use std::time::Duration;

use http_server::handler::start_server;
use tauri::{ AppHandle, Emitter };
use tokio::sync::mpsc;
use tokio::time::sleep;

mod http_server;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            let (tx, mut rx): (
                mpsc::Sender<(u64, AppHandle)>,
                mpsc::Receiver<(u64, AppHandle)>,
            ) = mpsc::channel(32);

            tauri::async_runtime::spawn(async move {
                start_server(app_handle, tx).await;
            });

            tauri::async_runtime::spawn(async move {
                while let Some((total_time, app_handle)) = rx.recv().await {
                    let _ = app_handle.emit("addtime_handler", total_time);
                    let _ = app_handle.emit("timer_update", total_time);

                    let mut remaining_time = total_time * 10;

                    while remaining_time > 0 {
                        sleep(Duration::from_secs(1)).await;
                        remaining_time -= 1;
                        let _ = app_handle.emit("timer_update", remaining_time);
                    }

                    let _ = app_handle.emit("timer_done", "Timer completed!");
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
