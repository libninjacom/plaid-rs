
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "reversed")]
    Reversed,
}