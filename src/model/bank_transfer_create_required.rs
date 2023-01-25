
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferCreateRequired {
    pub access_token: String,
    pub account_id: String,
    pub amount: String,
    pub description: String,
    pub idempotency_key: String,
    pub iso_currency_code: String,
    pub network: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: BankTransferUser,
}
impl std::fmt::Display for BankTransferCreateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}