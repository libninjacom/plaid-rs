
use serde::{Serialize, Deserialize};
use super::{BankTransferDirection, BankTransferFailure};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferEvent {
    pub account_id: String,
    pub bank_transfer_amount: String,
    pub bank_transfer_id: String,
    pub bank_transfer_iso_currency_code: String,
    pub bank_transfer_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<BankTransferDirection>,
    pub event_id: i64,
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<BankTransferFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for BankTransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}