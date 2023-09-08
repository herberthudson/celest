#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use celest::{CargoEvent, EliteDangerousLogEvent, ED_FILES, JOURNAL_LOG};
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};
use regex::Regex;
use rev_buf_reader::RevBufReader;
use std::{fs::File, io::BufRead, path::Path, sync::mpsc, time::Duration};
use tauri::Manager;

mod celest;

#[tauri::command]
fn get_is_configured() -> bool {
    true
}

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_is_configured])
        .setup(|app| {
            let app_handle = app.handle();

            tauri::async_runtime::spawn(async move { connect_to_logs(&app_handle) });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn check_if_config_file_exist() -> bool {
    Path::new("config.toml").exists()
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
    let path = "/home/herbert/.local/share/Steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous";
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
            println!("{:?}", event);
        }
    }
}
