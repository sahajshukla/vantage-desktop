// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu};

fn main() {
    // Create the View menu with refresh and dev tools
    let refresh = CustomMenuItem::new("refresh".to_string(), "Refresh")
        .accelerator("CmdOrCtrl+R");
    let hard_refresh = CustomMenuItem::new("hard_refresh".to_string(), "Hard Refresh")
        .accelerator("CmdOrCtrl+Shift+R");
    let dev_tools = CustomMenuItem::new("dev_tools".to_string(), "Toggle Developer Tools")
        .accelerator("CmdOrCtrl+Shift+I");
    let go_home = CustomMenuItem::new("go_home".to_string(), "Go to Home")
        .accelerator("CmdOrCtrl+H");

    let view_menu = Submenu::new(
        "View",
        Menu::new()
            .add_item(refresh)
            .add_item(hard_refresh)
            .add_native_item(MenuItem::Separator)
            .add_item(dev_tools)
            .add_native_item(MenuItem::Separator)
            .add_item(go_home),
    );

    // Build the full menu
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll)
        .add_native_item(MenuItem::Separator)
        .add_submenu(view_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            let window = event.window();
            match event.menu_item_id() {
                "refresh" => {
                    window.eval("window.location.reload()").ok();
                }
                "hard_refresh" => {
                    window.eval("window.location.reload(true)").ok();
                }
                "dev_tools" => {
                    #[cfg(debug_assertions)]
                    {
                        if window.is_devtools_open() {
                            window.close_devtools();
                        } else {
                            window.open_devtools();
                        }
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        // In release builds, still allow dev tools for debugging
                        if window.is_devtools_open() {
                            window.close_devtools();
                        } else {
                            window.open_devtools();
                        }
                    }
                }
                "go_home" => {
                    window.eval("window.location.href = 'https://app.vantageauditos.com'").ok();
                }
                _ => {}
            }
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // Load the Vantage AuditOS web app directly
            // The web app handles license keys and Clerk authentication
            window.eval("window.location.href = 'https://app.vantageauditos.com'").ok();
            window.show().ok();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
