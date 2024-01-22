use serde::{Serialize, Deserialize};
use super::ClientProvidedEnhancedTransaction;
///TransactionsEnhanceGetResponse defines the response schema for `/beta/transactions/v1/enhance`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnhanceGetResponse {
    ///An array of enhanced transactions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enhanced_transactions: Vec<ClientProvidedEnhancedTransaction>,
}
impl std::fmt::Display for TransactionsEnhanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}