
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRepaymentReturnListResponse {
    pub repayment_returns: Vec<TransferRepaymentReturn>,
    pub request_id: String,
}
impl std::fmt::Display for TransferRepaymentReturnListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}