
use serde::{Serialize, Deserialize};
use super::StatementsAccount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatementsListResponse {
    pub accounts: Vec<StatementsAccount>,
    pub institution_id: String,
    pub institution_name: String,
    pub item_id: String,
    pub request_id: String,
}
impl std::fmt::Display for StatementsListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}