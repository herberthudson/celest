#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use celest::{CargoEvent, EliteDangerousLogEvent, ED_FILES, JOURNAL_LOG};
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};
use regex::Regex;
use rev_buf_reader::RevBufReader;
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::BufRead, path::Path, sync::mpsc, time::Duration};
use tauri::Manager;
mod celest;

#[tauri::command]
fn is_configured(appHandle: tauri::AppHandle) -> bool {
    match config_exist(appHandle) {
        Ok(_) => true,
        Err(_) => false,
    }
}

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

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ConfigApp {
    ed_log_dir: String,
}

fn config_exist(app_handle: tauri::AppHandle) -> Result<ConfigApp, bool> {
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

fn valid_ed_logs_files(path: &str, partner: &str) -> bool {
    let regex_logs_files: Regex = Regex::new(partner).unwrap();

    regex_logs_files.is_match(path)
}

fn read_journal_log(path: &str) -> Vec<String> {
    let limit = 10;
    let file = File::open(path).expect("no such file");
    let buf = RevBufReader::new(file);
    buf.lines()
        .take(limit)
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

enum EDEvents {
    SendText,
    Cargo,
}

fn connect_to_logs(app_handle: &tauri::AppHandle) {
    let path = "/Users/herberthudson/Downloads/Journal.2022-11-16T141331.01.log";
    let (tx, rx) = mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new(path), RecursiveMode::Recursive)
        .unwrap();

    debouncer.cache().add_root(path, RecursiveMode::Recursive);

    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| match event {
                DebouncedEvent { event, time } => match event {
                    DataChange => {
                        let log_path = event.paths[0].to_str().unwrap();
                        if valid_ed_logs_files(log_path, ED_FILES)
                            || valid_ed_logs_files(log_path, JOURNAL_LOG)
                        {
                            let content = read_journal_log(log_path);
                            let last_value = content.get(0).unwrap().clone();
                            let ed_log_event: EliteDangerousLogEvent =
                                serde_json::from_str(&last_value).unwrap();
                            process_log_event(ed_log_event, &last_value, app_handle);
                        }
                    }
                },
            }),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn process_log_event(event: EliteDangerousLogEvent, value: &str, app_handle: &tauri::AppHandle) {
    match event.event.as_str() {
        "Cargo" => {
            let cargo: CargoEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("CargoEvent", cargo.clone());
        }
        _ => {
            println!("Event not found: {:?}", event);
        }
    }
}
