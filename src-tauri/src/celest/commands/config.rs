use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigApp {
    ed_log_dir: String,
}

#[tauri::command]
pub fn is_configured(appHandle: tauri::AppHandle) -> bool {
    match config_exist(appHandle) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn config_exist(app_handle: tauri::AppHandle) -> Result<ConfigApp, bool> {
    let path = app_handle.path_resolver().app_config_dir();
    match path {
        Some(mut path) => {
            path.push("config.json");
            println!("{:?}", path);
            match path.exists() {
                true => match load_config(&path.to_str().unwrap()) {
                    Ok(config) => Ok(config),
                    Err(e) => {
                        println!("Error on load config: {:?}", e);
                        Err(false)
                    }
                },
                false => Err(false),
            }
        }
        None => Err(false),
    }
}

fn load_config(path: &str) -> Result<ConfigApp, serde_json::Error> {
    let file = File::open(path).expect("no such file");
    match serde_json::from_reader(file) {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    }
}
