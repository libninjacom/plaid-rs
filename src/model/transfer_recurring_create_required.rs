
use serde::{Serialize, Deserialize};
use super::{TransferDevice, TransferRecurringSchedule, TransferUserInRequest};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringCreateRequired {
    pub access_token: String,
    pub account_id: String,
    pub amount: String,
    pub description: String,
    pub device: TransferDevice,
    pub idempotency_key: String,
    pub network: String,
    pub schedule: TransferRecurringSchedule,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: TransferUserInRequest,
}
impl std::fmt::Display for TransferRecurringCreateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}