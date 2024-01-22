use serde::{Serialize, Deserialize};
use super::TransactionsRuleDetails;
///A representation of a transactions category rule.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsCategoryRule {
    ///Date and time when a rule was created in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///A unique identifier of the rule created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A unique identifier of the Item the rule was created for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /**Personal finance category unique identifier.

In the personal finance category taxonomy, this field is represented by the detailed category field.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<String>,
    ///A representation of transactions rule details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_details: Option<TransactionsRuleDetails>,
}
impl std::fmt::Display for TransactionsCategoryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}