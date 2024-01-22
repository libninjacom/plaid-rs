use serde::{Serialize, Deserialize};
///International Bank Account Number for a Wallet Transaction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionCounterpartyInternational {
    ///International Bank Account Number (IBAN).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
}
impl std::fmt::Display for WalletTransactionCounterpartyInternational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}