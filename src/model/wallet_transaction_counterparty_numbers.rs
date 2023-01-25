
use serde::{Serialize, Deserialize};
use super::{
    WalletTransactionCounterpartyBacs, WalletTransactionCounterpartyInternational,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionCounterpartyNumbers {
    pub bacs: Option<WalletTransactionCounterpartyBacs>,
    pub international: Option<WalletTransactionCounterpartyInternational>,
}
impl std::fmt::Display for WalletTransactionCounterpartyNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}