
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "returned")]
    Returned,
}