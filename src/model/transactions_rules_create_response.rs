
use serde::{Serialize, Deserialize};
use super::TransactionsCategoryRule;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRulesCreateResponse {
    pub request_id: String,
    pub rule: TransactionsCategoryRule,
}
impl std::fmt::Display for TransactionsRulesCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}