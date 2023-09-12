use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewCommander {
    timestamp: TimeStamp,
    event: String, // NewCommander
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Package")]
    package: String, // Selected starter package
}
