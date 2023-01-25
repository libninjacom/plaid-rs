
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum LinkDeliverySessionStatus {
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "OPENED")]
    Opened,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "EXPIRED")]
    Expired,
}