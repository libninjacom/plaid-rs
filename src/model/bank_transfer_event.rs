
use serde::{Serialize, Deserialize};
use super::{BankTransferDirection, BankTransferFailure};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferEvent {
    pub account_id: String,
    pub bank_transfer_amount: String,
    pub bank_transfer_id: String,
    pub bank_transfer_iso_currency_code: String,
    pub bank_transfer_type: String,
    pub direction: Option<BankTransferDirection>,
    pub event_id: i64,
    pub event_type: String,
    pub failure_reason: Option<BankTransferFailure>,
    pub origination_account_id: Option<String>,
    pub timestamp: String,
}
impl std::fmt::Display for BankTransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}