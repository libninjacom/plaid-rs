
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationDecision {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "declined")]
    Declined,
}