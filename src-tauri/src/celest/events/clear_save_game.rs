use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClearSavedGame {
    timestamp: TimeStamp,
    event: String, // ClearSavedGame
    #[serde(rename = "Name")]
    name: String, // Commander name
    #[serde(rename = "FID")]
    fid: String, // Player id
}
