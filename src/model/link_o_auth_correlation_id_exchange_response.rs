
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkOAuthCorrelationIdExchangeResponse {
    pub link_token: String,
    pub request_id: String,
}
impl std::fmt::Display for LinkOAuthCorrelationIdExchangeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}