
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletListResponse {
    pub next_cursor: Option<String>,
    pub request_id: String,
    pub wallets: Vec<Wallet>,
}
impl std::fmt::Display for WalletListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}