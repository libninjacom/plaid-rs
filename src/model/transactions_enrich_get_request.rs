
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnrichGetRequest {
    pub account_type: String,
    pub options: Option<TransactionsEnrichRequestOptions>,
    pub transactions: Vec<ClientProvidedTransaction>,
}
impl std::fmt::Display for TransactionsEnrichGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}