
use serde::{Serialize, Deserialize};
use super::{AccountIdentityMatchScore, Item};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityMatchResponse {
    pub accounts: Vec<AccountIdentityMatchScore>,
    pub item: Item,
    pub request_id: String,
}
impl std::fmt::Display for IdentityMatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}