
use serde::{Serialize, Deserialize};
use super::TransactionsRuleDetails;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsCategoryRule {
    pub created_at: Option<String>,
    pub id: Option<String>,
    pub item_id: Option<String>,
    pub personal_finance_category: Option<String>,
    pub rule_details: Option<TransactionsRuleDetails>,
}
impl std::fmt::Display for TransactionsCategoryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}