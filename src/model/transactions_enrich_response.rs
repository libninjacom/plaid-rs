
use serde::{Serialize, Deserialize};
use super::ClientProvidedEnrichedTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnrichResponse {
    pub enriched_transactions: Vec<ClientProvidedEnrichedTransaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for TransactionsEnrichResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}