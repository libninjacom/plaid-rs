
use serde::{Serialize, Deserialize};
use super::ClientProvidedEnrichedTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnrichGetResponse {
    pub enriched_transactions: Vec<ClientProvidedEnrichedTransaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for TransactionsEnrichGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}