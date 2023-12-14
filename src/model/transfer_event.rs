
use serde::{Serialize, Deserialize};
use super::TransferFailure;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    pub event_id: i64,
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TransferFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<String>,
    pub transfer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}
impl std::fmt::Display for TransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}