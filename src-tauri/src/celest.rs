use serde::{Deserialize, Serialize};

pub const JOURNAL_LOG: &str = r"Journal(Alpha|Beta)?\.[0-9]{2,4}(-)?[0-9]{2}(-)?[0-9]{2}(T)?[0-9]{2}[0-9]{2}[0-9]{2}\.[0-9]{2}\.log$";
pub const ED_FILES: &str =
    r"(Cargo|Market|ModulesInfo|NavRoute|Outfitting|ShipLoker|Shipyard|Status)\.json$";
type TimeStamp = String; // TODO: change for date and time ISO 8601

#[derive(Debug, Deserialize)]
pub struct EliteDangerousLogEvent {
    timestamp: TimeStamp, // ISO 8601
    pub event: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileHeader {
    timestamp: TimeStamp,
    event: String,
    part: u16,
    language: String,
    #[serde(rename = "Odyssey")]
    odyssey: bool,
    gameversion: String,
    build: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CargoEvent {
    timestamp: TimeStamp, // ISO 8601
    event: String,        // Cargo
    #[serde(rename = "Vessel")]
    vessel: String, // Ship, SRV
    #[serde(rename = "Count")]
    count: u32,
    #[serde(rename = "Inventory")]
    inventory: Option<Vec<CargoInventory>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CargoInventory {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "Count")]
    count: u32,
    #[serde(rename = "Stolen")]
    stolen: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClearSavedGame {
    timestamp: TimeStamp,
    event: String, // ClearSavedGame
    #[serde(rename = "Name")]
    name: String, // Commander name
    #[serde(rename = "FID")]
    fid: String, // Player id
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewCommander {
    timestamp: TimeStamp,
    event: String, // NewCommander
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Package")]
    package: String, // Selected starter package
}
