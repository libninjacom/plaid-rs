
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountsBalanceGetRequestOptions {
    pub account_ids: Option<Vec<String>>,
    pub min_last_updated_datetime: Option<String>,
}
impl std::fmt::Display for AccountsBalanceGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}