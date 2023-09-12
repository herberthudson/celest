use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoadGameEvent {
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
