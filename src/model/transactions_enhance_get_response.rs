
use serde::{Serialize, Deserialize};
use super::ClientProvidedEnhancedTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnhanceGetResponse {
    pub enhanced_transactions: Vec<ClientProvidedEnhancedTransaction>,
}
impl std::fmt::Display for TransactionsEnhanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}