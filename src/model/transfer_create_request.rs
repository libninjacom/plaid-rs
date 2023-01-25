
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferCreateRequest {
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: Option<String>,
    pub authorization_id: String,
    pub description: String,
    pub idempotency_key: Option<String>,
    pub iso_currency_code: Option<String>,
    pub metadata: Option<TransferMetadata>,
    pub network: Option<String>,
    pub origination_account_id: Option<String>,
    pub payment_profile_token: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub user: Option<TransferUserInRequestDeprecated>,
}
impl std::fmt::Display for TransferCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}