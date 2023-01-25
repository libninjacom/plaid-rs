
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionListRequestOptions {
    pub end_time: Option<String>,
    pub start_time: Option<String>,
}
impl std::fmt::Display for WalletTransactionListRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}