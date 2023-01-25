
use serde::{Serialize, Deserialize};
use super::{AccountBase, Item, Transaction};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsGetResponse {
    pub accounts: Vec<AccountBase>,
    pub item: Item,
    pub request_id: String,
    pub total_transactions: i64,
    pub transactions: Vec<Transaction>,
}
impl std::fmt::Display for TransactionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}