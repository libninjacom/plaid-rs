
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentProfileStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "REMOVED")]
    Removed,
}