
use serde::{Serialize, Deserialize};
use super::TransferRefund;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefundCreateResponse {
    pub refund: TransferRefund,
    pub request_id: String,
}
impl std::fmt::Display for TransferRefundCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}