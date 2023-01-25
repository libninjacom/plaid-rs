
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRepaymentReturnListRequest {
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub repayment_id: String,
}
impl std::fmt::Display for TransferRepaymentReturnListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}