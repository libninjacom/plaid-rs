
use serde::{Serialize, Deserialize};
use super::TransferRefundFailure;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefund {
    pub amount: String,
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TransferRefundFailure>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_trace_id: Option<String>,
    pub status: String,
    pub transfer_id: String,
}
impl std::fmt::Display for TransferRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}