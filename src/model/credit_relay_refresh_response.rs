
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditRelayRefreshResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_report_id: Option<String>,
    pub relay_token: String,
    pub request_id: String,
}
impl std::fmt::Display for CreditRelayRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}