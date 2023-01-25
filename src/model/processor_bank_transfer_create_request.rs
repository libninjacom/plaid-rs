
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorBankTransferCreateRequest {
    pub ach_class: Option<String>,
    pub amount: String,
    pub custom_tag: Option<String>,
    pub description: String,
    pub idempotency_key: String,
    pub iso_currency_code: String,
    pub metadata: Option<BankTransferMetadata>,
    pub network: String,
    pub origination_account_id: Option<String>,
    pub processor_token: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: BankTransferUser,
}
impl std::fmt::Display for ProcessorBankTransferCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}