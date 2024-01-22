use serde::{Serialize, Deserialize};
///The amount and currency of a transaction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionAmount {
    ///An ISO-4217 currency code, used with e-wallets and transactions.
    pub iso_currency_code: String,
    ///The amount of the transaction. Must contain at most two digits of precision e.g. `1.23`.
    pub value: f64,
}
impl std::fmt::Display for WalletTransactionAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}