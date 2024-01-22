use serde::{Serialize, Deserialize};
///Identifying information for transferring money to or from an international bank account via wire transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersInternational {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The Bank Identifier Code (BIC) for the account
    pub bic: String,
    ///The International Bank Account Number (IBAN) for the account
    pub iban: String,
}
impl std::fmt::Display for NumbersInternational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}