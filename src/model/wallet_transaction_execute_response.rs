
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionExecuteResponse {
    pub request_id: String,
    pub status: String,
    pub transaction_id: String,
}
impl std::fmt::Display for WalletTransactionExecuteResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}