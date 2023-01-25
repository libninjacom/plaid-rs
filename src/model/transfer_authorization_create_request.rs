
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateRequest {
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: String,
    pub beacon_session_id: Option<String>,
    pub device: Option<TransferAuthorizationDevice>,
    pub idempotency_key: Option<String>,
    pub iso_currency_code: Option<String>,
    pub network: String,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub payment_profile_token: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: TransferAuthorizationUserInRequest,
    pub user_present: Option<bool>,
    pub with_guarantee: Option<bool>,
}
impl std::fmt::Display for TransferAuthorizationCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}