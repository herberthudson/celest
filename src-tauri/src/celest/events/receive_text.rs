use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReceiveText {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Message_Localised")]
    message_localised: String,
    #[serde(rename = "Channel")]
    channel: String,
}
