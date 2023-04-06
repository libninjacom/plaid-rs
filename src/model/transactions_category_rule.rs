
use serde::{Serialize, Deserialize};
use super::TransactionsRuleDetails;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsCategoryRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_details: Option<TransactionsRuleDetails>,
}
impl std::fmt::Display for TransactionsCategoryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}