use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

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
