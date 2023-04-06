
use serde::{Serialize, Deserialize};
use super::Wallet;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub request_id: String,
    pub wallets: Vec<Wallet>,
}
impl std::fmt::Display for WalletListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}