
use serde::{Serialize, Deserialize};
use super::WalletTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub request_id: String,
    pub transactions: Vec<WalletTransaction>,
}
impl std::fmt::Display for WalletTransactionListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}