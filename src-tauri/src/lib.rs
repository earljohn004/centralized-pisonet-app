use std::time::Duration;

use anyhow::Result;
use http_server::handler::start_server;
use settings::uuidmodel::UniqueId;
use tauri::menu::{ Menu, MenuItem };
use tauri::{ AppHandle, Emitter, Manager };
use tauri::tray::TrayIconBuilder;
use tokio::sync::mpsc;
use tokio::time::sleep;

mod http_server;
mod window_manager;
mod settings;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn create_system_tray(app: &AppHandle) -> Result<()> {
    let show_main_i = MenuItem::with_id(app, "show_main", "Show Main Window", true, None::<&str>)?;
    let show_small_i = MenuItem::with_id(
        app,
        "show_small",
        "Show Small Window",
        true,
        None::<&str>
    )?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&quit_i, &show_small_i, &show_main_i])?;
    let _ = TrayIconBuilder::new()
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "quit" => {
                    println!("quit menu item was clicked");
                    app.exit(0);
                }
                "show_small" => {
                    println!("show small was clicked");
                    window_manager::utility::show_small_window(app);
                    if let Some(small_window) = app.get_webview_window("small") {
                        small_window.show().unwrap();
                    }
                }
                "show_main" => {
                    println!("show main was clicked");
                    window_manager::utility::show_main_window(app);
                }
                _ => {
                    println!("menu item {:?} not handled", event.id);
                }
            }
        })
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .build(app);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .setup(|app| {
            //
            // Initialize the appconfig.json file
            //
            let application_config = settings::appconfig::initialize()?;

            //
            // Clone the app handle
            //
            let app_handle = app.handle().clone();

            //
            // Create the System Tray
            //
            let _ = create_system_tray(&app_handle)?;

            let (tx, mut rx): (
                mpsc::Sender<(u64, AppHandle)>,
                mpsc::Receiver<(u64, AppHandle)>,
            ) = mpsc::channel(32);

            let device = UniqueId::default()?;
            let device_name = device.id;

            let ip = application_config.get_ip_address(device_name.as_str())?;
            let port: u16 = application_config.get_port(device_name.as_str())?.parse()?;

            tauri::async_runtime::spawn(async move {
                let _ = start_server(app_handle, tx, ip, port).await;
            });

            tauri::async_runtime::spawn(async move {
                while let Some((total_time, app_handle)) = rx.recv().await {
                    let mut remaining_time = total_time * 5;

                    let _ = app_handle.emit("addtime_handler", total_time);
                    let _ = app_handle.emit("timer_update", remaining_time);

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
