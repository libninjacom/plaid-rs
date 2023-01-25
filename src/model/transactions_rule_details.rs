
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsRuleDetails {
    pub field: String,
    pub query: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TransactionsRuleDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}