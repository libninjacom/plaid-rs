
use serde::{Serialize, Deserialize};
use super::TransferFailure;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEvent {
    pub account_id: String,
    pub event_id: i64,
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TransferFailure>,
    pub funding_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweep_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweep_id: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub transfer_amount: String,
    pub transfer_id: String,
    pub transfer_type: String,
}
impl std::fmt::Display for TransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}