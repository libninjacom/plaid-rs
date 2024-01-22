use serde::{Serialize, Deserialize};
///Details about the end user's device
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalDevice {
    ///The IP address of the device that initiated the transaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The user agent of the device that initiated the transaction (e.g. "Mozilla/5.0")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl std::fmt::Display for SignalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}