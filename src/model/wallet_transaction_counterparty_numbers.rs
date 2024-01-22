use serde::{Serialize, Deserialize};
use super::{
    WalletTransactionCounterpartyBacs, WalletTransactionCounterpartyInternational,
};
///The counterparty's bank account numbers. Exactly one of IBAN or BACS data is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionCounterpartyNumbers {
    ///The account number and sort code of the counterparty's account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<WalletTransactionCounterpartyBacs>,
    ///International Bank Account Number for a Wallet Transaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international: Option<WalletTransactionCounterpartyInternational>,
}
impl std::fmt::Display for WalletTransactionCounterpartyNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}