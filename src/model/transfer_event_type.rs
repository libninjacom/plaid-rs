
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferEventType {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "returned")]
    Returned,
    #[serde(rename = "swept")]
    Swept,
    #[serde(rename = "swept_settled")]
    SweptSettled,
    #[serde(rename = "return_swept")]
    ReturnSwept,
}