
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkOAuthCorrelationIdExchangeRequest {
    pub link_correlation_id: String,
}
impl std::fmt::Display for LinkOAuthCorrelationIdExchangeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}