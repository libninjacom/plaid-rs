
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentCreateRequest {
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: String,
    pub description: String,
    pub iso_currency_code: Option<String>,
    pub metadata: Option<TransferMetadata>,
    pub mode: String,
    pub origination_account_id: Option<String>,
    pub require_guarantee: Option<bool>,
    pub user: TransferUserInRequest,
}
impl std::fmt::Display for TransferIntentCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}