
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorStripeBankAccountTokenCreateResponse {
    pub request_id: String,
    pub stripe_bank_account_token: String,
}
impl std::fmt::Display for ProcessorStripeBankAccountTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}