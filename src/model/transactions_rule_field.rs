
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionsRuleField {
    #[serde(rename = "TRANSACTION_ID")]
    TransactionId,
    #[serde(rename = "NAME")]
    Name,
}