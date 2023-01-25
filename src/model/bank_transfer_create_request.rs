
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<String>,
    pub amount: String,
    pub custom_tag: Option<String>,
    pub description: String,
    pub idempotency_key: String,
    pub iso_currency_code: String,
    pub metadata: Option<BankTransferMetadata>,
    pub network: String,
    pub origination_account_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: BankTransferUser,
}
impl std::fmt::Display for BankTransferCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}