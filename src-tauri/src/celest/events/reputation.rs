use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

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
