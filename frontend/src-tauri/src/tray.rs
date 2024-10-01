use tauri::{
    AppHandle, CustomMenuItem, Manager, PhysicalPosition, PhysicalSize, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder
};
use tauri_plugin_positioner::{WindowExt, Position};

pub fn create_tray() -> SystemTray {
    let calendar = CustomMenuItem::new("calendar".to_string(), "Open Calendar");
    let tasks = CustomMenuItem::new("date-converter".to_string(), "AD/BS Date Converter");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(calendar)
        .add_item(tasks)
        .add_item(settings)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu).with_tooltip("Miti Calendar")
}

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            toggle_window(app);
        }
        SystemTrayEvent::RightClick { .. } => {
            // Explicitly show the menu on right-click
            app.tray_handle().get_item("calendar").set_enabled(true).unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "calendar" => {
                    show_window_with_route(app, "calendar");
                }
                "date-converter" => {
                    open_date_converter(app)
                }
                "settings" => {
                    show_window_with_route(app, "app-settings");
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn toggle_window(app: &AppHandle) {
    if let Some(window) = app.get_window("main") {
        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            show_window(app);
        }
    }
}

fn show_window(app: &AppHandle) {
    let main_window = app.get_window("tray");
    match main_window {
        Some(window) => {
            let _ = window.move_window(Position::BottomRight);
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        _none => {
            let window = WindowBuilder::new(
                app,
                "tray",
                tauri::WindowUrl::App("calendar".into())
            )
            .title("Miti Calendar")
            .inner_size(600.0, 670.0)
            .resizable(true)
            .decorations(false)
            .skip_taskbar(true)
            .focused(true)
            .always_on_top(true)
            .build()
            .unwrap();
            let _ = window.move_window(Position::BottomRight);
            window.show().unwrap();
            window.set_focus().unwrap();

        }
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  route: String,
  window_id: String
}

fn show_window_with_route(app: &AppHandle, route: &str) {
    show_window(app);
    if let Some(window) = app.get_window("main") {
        println!("Navigating to route: {}", route);
        window.emit("navigate", Payload {
            route: route.to_string(),
            window_id: "main".to_string()
        }).unwrap();
    }
}

fn open_date_converter(app: &AppHandle) {
    let date_converter_window = app.get_window("date_converter");
    match date_converter_window {
        Some(window) => {
            window.show().unwrap();
            window.set_focus().unwrap();
            let _ = window.move_window(Position::BottomRight);
        }
        _none => {
            let window = WindowBuilder::new(
                app,
                "date_converter",
                tauri::WindowUrl::App("converter".into())
            )
            .title("Date Converter")
            .inner_size(400.0, 500.0)
            .decorations(false)
            .focused(true)
            .always_on_top(true)
            .skip_taskbar(true)
            .build()
            .unwrap();
        let _ = window.move_window(Position::BottomRight);
        }
    }
}