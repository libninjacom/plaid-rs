
use serde::{Serialize, Deserialize};
use super::StatementsStatement;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatementsAccount {
    pub account_id: String,
    pub account_name: String,
    pub account_type: String,
    pub statements: Vec<StatementsStatement>,
}
impl std::fmt::Display for StatementsAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}