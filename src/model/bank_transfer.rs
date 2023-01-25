
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransfer {
    pub account_id: String,
    pub ach_class: String,
    pub amount: String,
    pub cancellable: bool,
    pub created: String,
    pub custom_tag: Option<String>,
    pub description: String,
    pub direction: Option<BankTransferDirection>,
    pub failure_reason: Option<BankTransferFailure>,
    pub id: String,
    pub iso_currency_code: String,
    pub metadata: Option<BankTransferMetadata>,
    pub network: String,
    pub origination_account_id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: BankTransferUser,
}
impl std::fmt::Display for BankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}