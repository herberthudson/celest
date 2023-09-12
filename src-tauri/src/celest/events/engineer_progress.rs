use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EngineersProgressEvent {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Engineer")]
    engineer: Vec<Engineer>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Engineer {
    #[serde(rename = "Engineer")]
    engineer: String,
    #[serde(rename = "EngineerID")]
    engineer_id: u64,
    #[serde(rename = "Progress")]
    progress: String,
}
