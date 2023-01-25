
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountsBalanceGetRequest {
    pub access_token: String,
    pub options: Option<AccountsBalanceGetRequestOptions>,
}
impl std::fmt::Display for AccountsBalanceGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}