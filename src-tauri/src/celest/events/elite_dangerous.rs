use serde::Deserialize;

pub type TimeStamp = String; // TODO: change for date and time ISO 8601

#[derive(Debug, Deserialize)]
pub struct EliteDangerousLogEvent {
    timestamp: TimeStamp, // ISO 8601
    pub event: String,
}
