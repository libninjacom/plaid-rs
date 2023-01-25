
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<String>,
    pub amount: String,
    pub description: String,
    pub device: TransferDevice,
    pub idempotency_key: String,
    pub iso_currency_code: Option<String>,
    pub network: String,
    pub schedule: TransferRecurringSchedule,
    pub test_clock_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: TransferUserInRequest,
    pub user_present: Option<bool>,
}
impl std::fmt::Display for TransferRecurringCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}