
use serde::{Serialize, Deserialize};
use super::{RemovedTransaction, Transaction};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTransactionsSyncResponse {
    pub added: Vec<Transaction>,
    pub has_more: bool,
    pub modified: Vec<Transaction>,
    pub next_cursor: String,
    pub removed: Vec<RemovedTransaction>,
    pub request_id: String,
}
impl std::fmt::Display for ProcessorTransactionsSyncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}