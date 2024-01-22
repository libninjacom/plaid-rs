use serde::{Serialize, Deserialize};
///Identifying information for transferring holdings to an investments account via ACATS.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersAcats {
    ///The full account number for the account
    pub account: String,
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///Identifiers for the clearinghouses that are assocciated with the account in order of relevance. This array will be empty if we can't provide any account level data. Institution level data can be retrieved from the institutions/get endpoints.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtc_numbers: Vec<String>,
}
impl std::fmt::Display for NumbersAcats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}