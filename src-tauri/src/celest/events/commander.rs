use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Commander {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "FID")]
    fid: String,
}
