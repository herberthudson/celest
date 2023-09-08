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
pub struct Commander {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "FID")]
    fid: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Materials {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Raw")]
    raw: Vec<MaterialsQuantity>,
    #[serde(rename = "Manufactured")]
    manufactured: Vec<MaterialsQuantity>,
    #[serde(rename = "Encoded")]
    encoded: Vec<MaterialsQuantity>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MaterialsQuantity {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Count")]
    count: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Rank {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Combat")]
    combat: u8,
    #[serde(rename = "Trade")]
    trade: u8,
    #[serde(rename = "Explore")]
    explore: u8,
    #[serde(rename = "Soldier")]
    soldier: u8,
    #[serde(rename = "Exobiologist")]
    exobiologist: u8,
    #[serde(rename = "Empire")]
    empire: u8,
    #[serde(rename = "Federation")]
    federation: u8,
    #[serde(rename = "CQC")]
    cqc: u8,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Progress {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Combat")]
    combat: u8,
    #[serde(rename = "Trade")]
    trade: u8,
    #[serde(rename = "Explore")]
    explore: u8,
    #[serde(rename = "Soldier")]
    soldier: u8,
    #[serde(rename = "Exobiologist")]
    exobiologist: u8,
    #[serde(rename = "Empire")]
    empire: u8,
    #[serde(rename = "Federation")]
    federation: u8,
    #[serde(rename = "CQC")]
    cqc: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reputation {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Empire")]
    empire: f32,
    #[serde(rename = "Federation")]
    federation: f32,
    #[serde(rename = "Alliance")]
    alliance: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EngineersProgrees {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Engineer")]
    engineer: Vec<Engineer>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Engineer {
    #[serde(rename = "Engineer")]
    engineer: String,
    #[serde(rename = "EngineerID")]
    engineer_id: u64,
    #[serde(rename = "Progress")]
    progress: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoadGame {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "FID")]
    fid: String,
    #[serde(rename = "Commander")]
    commander: String,
    #[serde(rename = "Horizons")]
    horizons: bool,
    #[serde(rename = "Odyssey")]
    odyssey: bool,
    #[serde(rename = "Ship")]
    ship: String,
    #[serde(rename = "Ship_Localised")]
    ship_localised: String,
    #[serde(rename = "ShipID")]
    ship_id: u16,
    #[serde(rename = "ShipName")]
    ship_name: String,
    #[serde(rename = "ShipIdent")]
    ship_ident: String,
    #[serde(rename = "FuelLevel")]
    fuel_level: f32,
    #[serde(rename = "FuelCapacity")]
    fuel_capacity: f32,
    #[serde(rename = "StartLanded")]
    start_landed: bool,
    #[serde(rename = "GameMode")]
    game_mode: String,
    #[serde(rename = "Group")]
    group: String,
    #[serde(rename = "Credits")]
    credits: u64,
    #[serde(rename = "Loan")]
    loan: u64,
    #[serde(rename = "language")]
    language: String,
    #[serde(rename = "gameversion")]
    gameversion: String,
    #[serde(rename = "build")]
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
