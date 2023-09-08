use serde::{Deserialize, Serialize};

pub const JOURNAL_LOG: &str = r"Journal(Alpha|Beta)?\.[0-9]{2,4}(-)?[0-9]{2}(-)?[0-9]{2}(T)?[0-9]{2}[0-9]{2}[0-9]{2}\.[0-9]{2}\.log$";
pub const ED_FILES: &str =
    r"(Cargo|Market|ModulesInfo|NavRoute|Outfitting|ShipLoker|Shipyard|Status)\.json$";

#[derive(Debug, Deserialize)]
pub struct EliteDangerousLogEvent {
    timestamp: String, // ISO 8601
    pub event: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CargoEvent {
    timestamp: String, // ISO 8601
    event: String,     // Cargo
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
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Count")]
    count: u32,
    #[serde(rename = "Stolen")]
    stolen: u32,
}
