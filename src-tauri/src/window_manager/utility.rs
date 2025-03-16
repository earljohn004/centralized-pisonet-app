use tauri::{ AppHandle, Manager };

pub fn show_small_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let new_width = 400.0;
        let new_height = 300.0;

        if let Ok(Some(monitor)) = window.primary_monitor() {
            let screen_size = monitor.size();

            let x = (screen_size.width as f64) - new_width - 30.0; // 10px margin from right
            let y = (screen_size.height as f64) - new_height - 80.0; // 10px margin from bottom

            window
                .set_size(
                    tauri::Size::Physical(tauri::PhysicalSize {
                        width: new_width as u32,
                        height: new_height as u32,
                    })
                )
                .unwrap();

            window
                .set_position(
                    tauri::Position::Physical(tauri::PhysicalPosition { x: x as i32, y: y as i32 })
                )
                .unwrap();

            window.show().unwrap();
        } else {
            println!("Failed to get primary monitor size.");
        }
    }
}

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.maximize().unwrap();
        window.show().unwrap();
    }
}
