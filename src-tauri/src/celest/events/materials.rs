use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MaterialsEvent {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Raw")]
    raw: Vec<MaterialsQuantity>,
    #[serde(rename = "Manufactured")]
    manufactured: Vec<MaterialsQuantity>,
    #[serde(rename = "Encoded")]
    encoded: Vec<MaterialsQuantity>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct MaterialsQuantity {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Count")]
    count: u16,
}
