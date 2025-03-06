// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/*
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_positioner::init())
        .on_tray_icon_event(|tray_handle, event| {
            tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
        })
        // .manage(CookieStore(Mutex::new(None))) // Initialize CookieStore here
        .invoke_handler(tauri::generate_handler![greet,]);

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
