
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferEvent {
    pub account_id: String,
    pub event_id: i64,
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub refund_id: Option<String>,
    pub sweep_amount: Option<String>,
    pub sweep_id: Option<String>,
    pub timestamp: String,
    pub transfer_amount: String,
    pub transfer_id: String,
    pub transfer_type: String,
}
impl std::fmt::Display for TransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}