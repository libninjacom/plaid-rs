use serde::{Serialize, Deserialize};
///Identifying information for transferring money to or from a UK bank account via BACS.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersBacs {
    ///The BACS account number for the account
    pub account: String,
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The BACS sort code for the account
    pub sort_code: String,
}
impl std::fmt::Display for NumbersBacs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}