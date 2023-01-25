
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthGetResponse {
    pub accounts: Vec<AccountBase>,
    pub item: Item,
    pub numbers: AuthGetNumbers,
    pub request_id: String,
}
impl std::fmt::Display for AuthGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}