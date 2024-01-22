use serde::{Serialize, Deserialize};
use super::TransactionsCategoryRule;
///TransactionsRulesCreateResponse defines the response schema for `/beta/transactions/rules/v1/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsRulesCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///A representation of a transactions category rule.
    pub rule: TransactionsCategoryRule,
}
impl std::fmt::Display for TransactionsRulesCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}