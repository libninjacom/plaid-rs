
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum WalletTransactionStatus {
    #[serde(rename = "AUTHORISING")]
    Authorising,
    #[serde(rename = "INITIATED")]
    Initiated,
    #[serde(rename = "EXECUTED")]
    Executed,
    #[serde(rename = "SETTLED")]
    Settled,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "FAILED")]
    Failed,
}