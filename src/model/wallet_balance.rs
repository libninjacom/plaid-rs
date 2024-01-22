use serde::{Serialize, Deserialize};
///An object representing the e-wallet balance
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletBalance {
    ///The total amount of funds in the account after subtracting pending debit transaction amounts
    pub available: f64,
    ///The total amount of funds in the account
    pub current: f64,
    ///The ISO-4217 currency code of the balance
    pub iso_currency_code: String,
}
impl std::fmt::Display for WalletBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}