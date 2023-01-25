
use serde::{Serialize, Deserialize};
use super::TransferRepayment;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRepaymentListResponse {
    pub repayments: Vec<TransferRepayment>,
    pub request_id: String,
}
impl std::fmt::Display for TransferRepaymentListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}