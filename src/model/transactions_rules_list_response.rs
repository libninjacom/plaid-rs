use serde::{Serialize, Deserialize};
use super::TransactionsCategoryRule;
///TransactionsRulesListResponse defines the response schema for `/beta/transactions/rules/v1/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsRulesListResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///A list of the Item's transaction rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<TransactionsCategoryRule>,
}
impl std::fmt::Display for TransactionsRulesListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}