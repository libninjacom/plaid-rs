
use serde::{Serialize, Deserialize};
use super::{WalletBalance, WalletNumbers};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub balance: WalletBalance,
    pub numbers: WalletNumbers,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    pub status: String,
    pub wallet_id: String,
}
impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}