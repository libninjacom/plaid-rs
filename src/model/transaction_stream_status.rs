
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStreamStatus {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "MATURE")]
    Mature,
    #[serde(rename = "EARLY_DETECTION")]
    EarlyDetection,
    #[serde(rename = "TOMBSTONED")]
    Tombstoned,
}