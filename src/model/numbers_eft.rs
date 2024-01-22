use serde::{Serialize, Deserialize};
///Identifying information for transferring money to or from a Canadian bank account via EFT.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersEft {
    ///The EFT account number for the account
    pub account: String,
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The EFT branch number for the account
    pub branch: String,
    ///The EFT institution number for the account
    pub institution: String,
}
impl std::fmt::Display for NumbersEft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}