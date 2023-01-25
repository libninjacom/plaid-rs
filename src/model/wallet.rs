
use serde::{Serialize, Deserialize};
use super::{WalletBalance, WalletNumbers};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub balance: WalletBalance,
    pub numbers: WalletNumbers,
    pub recipient_id: Option<String>,
    pub wallet_id: String,
}
impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}