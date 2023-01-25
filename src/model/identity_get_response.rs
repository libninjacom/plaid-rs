
use serde::{Serialize, Deserialize};
use super::{AccountIdentity, Item};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityGetResponse {
    pub accounts: Vec<AccountIdentity>,
    pub item: Item,
    pub request_id: String,
}
impl std::fmt::Display for IdentityGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}