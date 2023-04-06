
use serde::{Serialize, Deserialize};
use super::{
    BankTransferDirection, BankTransferFailure, BankTransferMetadata, BankTransferUser,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransfer {
    pub account_id: String,
    pub ach_class: String,
    pub amount: String,
    pub cancellable: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tag: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<BankTransferDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<BankTransferFailure>,
    pub id: String,
    pub iso_currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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