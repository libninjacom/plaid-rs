
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_proxy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_spam_list_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_timezone_offset: Option<String>,
}
impl std::fmt::Display for RiskCheckDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}