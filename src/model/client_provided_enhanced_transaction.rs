use serde::{Serialize, Deserialize};
use super::Enhancements;
///A client-provided transaction that Plaid has enhanced.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientProvidedEnhancedTransaction {
    ///The value of the transaction, denominated in the account's currency, as stated in `iso_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    ///The raw description of the transaction.
    pub description: String,
    ///A grouping of the Plaid produced transaction enhancement fields.
    pub enhancements: Enhancements,
    ///Unique transaction identifier to tie transactions back to clients' systems.
    pub id: String,
    ///The ISO-4217 currency code of the transaction.
    pub iso_currency_code: String,
}
impl std::fmt::Display for ClientProvidedEnhancedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}