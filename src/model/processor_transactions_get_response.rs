
use serde::{Serialize, Deserialize};
use super::{AccountBase, Transaction};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTransactionsGetResponse {
    pub account: AccountBase,
    pub request_id: String,
    pub total_transactions: i64,
    pub transactions: Vec<Transaction>,
}
impl std::fmt::Display for ProcessorTransactionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}