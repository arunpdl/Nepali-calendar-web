mod tray;

use tauri_plugin_autostart::MacosLauncher;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        // .setup(|app| {
        //     let window = app.get_window("main").unwrap();
        //     window.set_skip_taskbar(true).unwrap();
        //     Ok(())
        // })
        .system_tray(tray::create_tray())
        .on_system_tray_event(|app, event| tray::handle_tray_event(app, event))
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
         
            if let tauri::WindowEvent::Focused(focused) = event.event() {
                if !focused {
                    event.window().hide().unwrap()
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}