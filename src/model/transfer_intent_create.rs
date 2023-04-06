
use serde::{Serialize, Deserialize};
use super::{TransferMetadata, TransferUserInResponse};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<String>,
    pub amount: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub funding_account_id: String,
    pub id: String,
    pub iso_currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TransferMetadata>,
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub origination_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_guarantee: Option<bool>,
    pub status: String,
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for TransferIntentCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}