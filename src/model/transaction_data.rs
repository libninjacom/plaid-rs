
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionData {
    pub account_id: String,
    pub amount: f64,
    pub date: String,
    pub description: String,
    pub transaction_id: String,
}
impl std::fmt::Display for TransactionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}