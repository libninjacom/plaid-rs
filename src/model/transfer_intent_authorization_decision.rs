
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferIntentAuthorizationDecision {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "DECLINED")]
    Declined,
}