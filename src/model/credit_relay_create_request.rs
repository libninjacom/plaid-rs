
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditRelayCreateRequest {
    pub report_tokens: Vec<String>,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl std::fmt::Display for CreditRelayCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}