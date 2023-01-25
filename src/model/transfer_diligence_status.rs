
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferDiligenceStatus {
    #[serde(rename = "under_review")]
    UnderReview,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
}