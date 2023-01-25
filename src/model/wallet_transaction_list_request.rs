
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionListRequest {
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub options: Option<WalletTransactionListRequestOptions>,
    pub wallet_id: String,
}
impl std::fmt::Display for WalletTransactionListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}