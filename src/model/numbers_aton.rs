use serde::{Serialize, Deserialize};
///Identifying information for transferring holdings to an investments account via ATON.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersAton {
    ///The full account number for the account
    pub account: String,
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
}
impl std::fmt::Display for NumbersAton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}