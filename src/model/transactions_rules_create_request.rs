
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRulesCreateRequest {
    pub access_token: String,
    pub personal_finance_category: String,
    pub rule_details: TransactionsRuleDetails,
}
impl std::fmt::Display for TransactionsRulesCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}