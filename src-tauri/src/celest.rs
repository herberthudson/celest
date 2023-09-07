use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct EliteDangerousLogEvent {
    timestamp: String,
    pub event: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CargoEvent {
    timestamp: String,
    event: String,
    #[serde(rename = "Vessel")]
    vessel: String,
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
