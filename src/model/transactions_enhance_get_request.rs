
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnhanceGetRequest {
    pub account_type: String,
    pub transactions: Vec<ClientProvidedRawTransaction>,
}
impl std::fmt::Display for TransactionsEnhanceGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}