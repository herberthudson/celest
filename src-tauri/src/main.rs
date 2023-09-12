#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod celest;
use celest::commands::config::{config_exist, is_configured};
use celest::logs::utils::connect_to_logs;

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_configured])
        .setup(|app| {
            let app_handle = app.handle();
            match config_exist(app_handle.clone()) {
                Ok(config) => {
                    println!("Config {:?}", config);
                    tauri::async_runtime::spawn(async move { connect_to_logs(&app_handle) });
                }
                Err(_) => {
                    println!("Config not found");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

enum EDEvents {
    SendText,
    Cargo,
}
