
use serde::{Serialize, Deserialize};
use super::TransferBalance;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBalanceGetResponse {
    pub balance: TransferBalance,
    pub request_id: String,
}
impl std::fmt::Display for TransferBalanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}