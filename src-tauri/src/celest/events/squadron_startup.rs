use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SquadronStartup {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "SquadronName")]
    squadron_name: String,
    #[serde(rename = "CurrentRank")]
    current_rank: u8,
}
