
use serde::{Serialize, Deserialize};
use super::{AccountBase, Item};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    pub accounts: Vec<AccountBase>,
    pub item: Item,
    pub request_id: String,
}
impl std::fmt::Display for AccountsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}