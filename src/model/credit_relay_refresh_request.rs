
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditRelayRefreshRequest {
    pub relay_token: String,
    pub report_type: String,
    pub webhook: Option<String>,
}
impl std::fmt::Display for CreditRelayRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}