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
            toggle_window(app, "calendar", (600.0, 680.0));
        }
        SystemTrayEvent::RightClick { .. } => {
            // Explicitly show the menu on right-click
            app.tray_handle().get_item("calendar").set_enabled(true).unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "calendar" => {
                    create_or_show_window(app, "calendar", (600.0, 680.0));
                }
                "date-converter" => {
                    create_or_show_window(app, "converter", (400.0, 500.0))
                }
                "settings" => {
                    create_or_show_window(app, "app-settings", (400.0, 500.0));
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

fn toggle_window(app: &AppHandle, route: &str, size: (f64, f64)) {
    if let Some(window) = app.get_window("tray") {
        // let _ = window.move_window(Position::TrayCenter);
        if window.is_visible().unwrap() {
            window.hide().unwrap();
            return;
        }
    }
    create_or_show_window(app, route, size);
}

fn create_or_show_window(app: &AppHandle, route: &str, size: (f64, f64)) {
    let window = app.get_window("tray");
    match window {
        Some(window) => {
            window.set_size(PhysicalSize::new(size.0, size.1)).unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        None => {
            let window = WindowBuilder::new(
                app,
                "tray",
                tauri::WindowUrl::App(route.into())  // Dynamic content based on route
            )
            .title("Miti Calendar")
            .inner_size(size.0, size.1)
            .resizable(true)
            .decorations(false)
            .skip_taskbar(true)
            .focused(true)
            .always_on_top(true)
            .build()
            .unwrap();

            // let _ = window.move_window(Position::TrayCenter);  // Positioning the window
            window.show().unwrap();
            window.set_focus().unwrap();
        }
    }

    // Dynamically set the content
    change_window_content(app, route);
}

fn change_window_content(app: &AppHandle, route: &str) {
    if let Some(window) = app.get_window("tray") {
        println!("Navigating to route: {}", route);
        window.emit("navigate", Payload {
            route: route.to_string(),
            window_id: "tray".to_string()
        }).unwrap();
    }
}

fn set_window_position(app: &AppHandle) {
    if let Some(window) = app.get_window("tray") {
        if let Err(e) = window.move_window(Position::TrayCenter) {
            println!("Error moving window to tray position: {}, falling back to default position", e);
            let _ = window.move_window(Position::BottomRight);
        }
        
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  route: String,
  window_id: String
}

