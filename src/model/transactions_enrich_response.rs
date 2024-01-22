use serde::{Serialize, Deserialize};
use super::ClientProvidedEnrichedTransaction;
///TransactionsEnrichResponse defines the response schema for `/transactions/enrich`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnrichResponse {
    ///A list of enriched transactions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enriched_transactions: Vec<ClientProvidedEnrichedTransaction>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for TransactionsEnrichResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}