
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorStripeBankAccountTokenCreateRequest {
    pub access_token: String,
    pub account_id: String,
}
impl std::fmt::Display for ProcessorStripeBankAccountTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}