// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
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
