use serde::{Serialize, Deserialize};
///A representation of transactions rule details.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsRuleDetails {
    ///Transaction field for which the rule is defined.
    pub field: String,
    ///For TRANSACTION_ID field, provide transaction_id. For NAME field, provide a string pattern.
    pub query: String,
    /**Transaction rule's match type. For TRANSACTION_ID field, EXACT_MATCH is available.
Matches are case sensitive.*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TransactionsRuleDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}