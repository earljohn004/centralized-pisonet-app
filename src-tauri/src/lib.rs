use std::sync::atomic::{ AtomicU64, Ordering };
use std::sync::Arc;
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
mod licensing;

type AppConfigState = std::sync::Mutex<settings::appconfigmodels::AppConfig>;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn authorize(
    serial_number: &str,
    email_address: &str,
    state: tauri::State<AppConfigState>
) -> bool {
    println!("Serial number: {}, email address: {}", serial_number, email_address);

    let async_result = tauri::async_runtime::block_on(async {
        licensing::cloud_service::authorize(serial_number, email_address, "mockdeviceid").await
    });

    if let Ok(result) = async_result {
        println!("Authorization result: {:?}", result);
        return result;
    } else {
        println!("Authorization error encountered!");
    }

    false
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
    let app_config = match settings::appconfig::initialize() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to initialize appconfig: {}", e);
            return;
        }
    };

    tauri::Builder
        ::default()
        .setup(|app| {
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

            //
            // Get the application config
            //
            let device = UniqueId::default()?;
            let device_name = device.id;
            let config = app.handle().state::<AppConfigState>();

            let ip: String;
            let port: u16;
            {
                let application_config = config.lock().unwrap();
                ip = application_config.get_ip_address(device_name.as_str())?;
                port = application_config.get_port(device_name.as_str())?.parse()?;
            }

            //
            // Thread to start the server
            //
            tauri::async_runtime::spawn(async move {
                let _ = start_server(app_handle.clone(), tx, ip, port).await;
            });

            let remaining_time = Arc::new(AtomicU64::new(0));
            let remaining_time_countdown = Arc::clone(&remaining_time);

            //
            // Thread to update ui with the remaining time
            //
            let countdown_app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    let current_remaining_time = remaining_time_countdown.load(Ordering::SeqCst);
                    if current_remaining_time == 0 {
                        window_manager::utility::show_main_window(&countdown_app_handle);
                    }

                    if current_remaining_time > 0 {
                        println!("Remaining time: {}", current_remaining_time);
                        let _ = countdown_app_handle.emit("timer_update", current_remaining_time);
                        remaining_time_countdown.fetch_sub(1, Ordering::SeqCst);
                        sleep(Duration::from_secs(1)).await;
                    } else {
                        let _ = countdown_app_handle.emit("timer_done", "Timer completed!");
                        sleep(Duration::from_millis(100)).await;
                    }
                }
            });

            //
            // Thread to receive timer events from the workers
            //
            tauri::async_runtime::spawn(async move {
                while let Some((total_time, app_handle)) = rx.recv().await {
                    println!("Received total time: {}", total_time);
                    remaining_time.fetch_add(total_time * 5, Ordering::SeqCst);
                    let _ = app_handle.emit("addtime_handler", total_time);

                    // Transition window to small when coin is inserted
                    window_manager::utility::show_small_window(&app_handle);
                    if let Some(small_window) = app_handle.get_webview_window("small") {
                        small_window.show().unwrap();
                    }
                }
            });

            Ok(())
        })
        .manage(std::sync::Mutex::new(app_config))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, authorize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
