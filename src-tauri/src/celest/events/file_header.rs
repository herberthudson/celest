use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

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
