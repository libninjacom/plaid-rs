
use serde::{Serialize, Deserialize};
use super::{TransferMetadata, TransferUserInResponse};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentCreate {
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: String,
    pub created: String,
    pub description: String,
    pub id: String,
    pub iso_currency_code: String,
    pub metadata: Option<TransferMetadata>,
    pub mode: String,
    pub origination_account_id: String,
    pub require_guarantee: Option<bool>,
    pub status: String,
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for TransferIntentCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}