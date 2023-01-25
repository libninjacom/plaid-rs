
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionCounterparty {
    pub name: String,
    pub numbers: WalletTransactionCounterpartyNumbers,
}
impl std::fmt::Display for WalletTransactionCounterparty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}