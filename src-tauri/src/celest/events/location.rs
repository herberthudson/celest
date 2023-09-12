use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationEvent {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Latitude")]
    latitude: f64,
    #[serde(rename = "Longitude")]
    longitude: f64,
    #[serde(rename = "DistFromStarLS")]
    dist_from_star_ls: f64,
    #[serde(rename = "Docked")]
    docked: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "StarPos")]
    star_pos: [f64; 3],
    #[serde(rename = "SystemAllegiance")]
    system_allegiance: String,
    #[serde(rename = "SystemEconomy")]
    system_economy: String,
    #[serde(rename = "SystemEconomy_Localised")]
    system_economy_localised: String,
    #[serde(rename = "SystemSecondEconomy")]
    system_second_economy: String,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    system_second_economy_localised: String,
    #[serde(rename = "SystemGovernment")]
    system_government: String,
    #[serde(rename = "SystemGovernment_Localised")]
    system_government_localised: String,
    #[serde(rename = "SystemSecurity")]
    system_security: String,
    #[serde(rename = "SystemSecurity_Localised")]
    system_security_localised: String,
    #[serde(rename = "Population")]
    population: u32,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u8,
    #[serde(rename = "BodyType")]
    body_type: String,
}
