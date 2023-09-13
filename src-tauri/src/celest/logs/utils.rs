use crate::celest::events::cargo::CargoEvent;
use crate::celest::events::clear_save_game::ClearSavedGameEvent;
use crate::celest::events::commander::CommanderEvent;
use crate::celest::events::elite_dangerous::EliteDangerousLogEvent;
use crate::celest::events::file_header::FileHeaderEvent;
use crate::celest::events::load_game::LoadGameEvent;
use crate::celest::events::location::LocationEvent;
use crate::celest::events::materials::MaterialsEvent;
use crate::celest::events::progress::ProgressEvent;
use crate::celest::events::rank::RankEvent;
use crate::celest::events::receive_text::ReceiveTextEvent;
use crate::celest::events::reputation::ReputationEvent;
use crate::celest::events::squadron_startup::SquadronStartupEvent;
use crate::celest::events::statistics::StatisticsEvent;

use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};

use regex::Regex;
use rev_buf_reader::RevBufReader;
use std::{fs::File, io::BufRead, path::Path, sync::mpsc, time::Duration};
use tauri::Manager;

const JOURNAL_LOG: &str = r"Journal(Alpha|Beta)?\.[0-9]{2,4}(-)?[0-9]{2}(-)?[0-9]{2}(T)?[0-9]{2}[0-9]{2}[0-9]{2}\.[0-9]{2}\.log$";
const ED_FILES: &str =
    r"(Cargo|Market|ModulesInfo|NavRoute|Outfitting|ShipLoker|Shipyard|Status)\.json$";

fn valid_ed_logs_files(path: &str, partner: &str) -> bool {
    let regex_logs_files: Regex = Regex::new(partner).unwrap();

    regex_logs_files.is_match(path)
}

fn read_journal_log(path: &str) -> Vec<String> {
    // TODO: valid what is the best limit for how many lines should be read
    let limit = 10;
    let file = File::open(path).expect("no such file");
    let buf = RevBufReader::new(file);
    buf.lines()
        .take(limit)
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

pub fn connect_to_logs(app_handle: &tauri::AppHandle) {
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
                        //TODO: Valid and process all paths or just the last
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
            let cargo_event: CargoEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("CargoEvent", cargo_event.clone());
        }
        "ClearSavedGameEvent" => {
            let clear_saved_game_event: ClearSavedGameEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("ClearSavedGameEvent", clear_saved_game_event.clone());
        }
        "CommanderEvent" => {
            let commander_event: CommanderEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("CommanderEvent", commander_event.clone());
        }
        "FileHeaderEvent" => {
            let file_header_event: FileHeaderEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("FileHeaderEvent", file_header_event.clone());
        }
        "LoadGameEvent" => {
            let load_game_event: LoadGameEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("LoadGameEvent", load_game_event.clone());
        }
        "LocationEvent" => {
            let location_event: LocationEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("LocationEvent", location_event.clone());
        }
        "MaterialsEvent" => {
            let materials_event: MaterialsEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("MaterialsEvent", materials_event.clone());
        }
        "NewCommanderEvent" => {
            let new_commander_event: CommanderEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("NewCommanderEvent", new_commander_event.clone());
        }
        "ProgressEvent" => {
            let progress_event: ProgressEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("ProgressEvent", progress_event.clone());
        }
        "RankEvent" => {
            let rank_event: RankEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("RankEvent", rank_event.clone());
        }
        "ReceiveTextEvent" => {
            let receive_text_event: ReceiveTextEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("ReceiveTextEvent", receive_text_event.clone());
        }
        "ReputationEvent" => {
            let reputation_event: ReputationEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("ReputationEvent", reputation_event.clone());
        }
        "SquadronStartupEvent" => {
            let squadron_startup_event: SquadronStartupEvent =
                serde_json::from_str(&value).unwrap();
            app_handle.emit_all("SquadronStartupEvent", squadron_startup_event.clone());
        }
        "StatisticsEvent" => {
            let statistics_event: StatisticsEvent = serde_json::from_str(&value).unwrap();
            app_handle.emit_all("StatisticsEvent", statistics_event.clone());
        }
        _ => {
            println!("Event not found: {:?}", event);
        }
    }
}
