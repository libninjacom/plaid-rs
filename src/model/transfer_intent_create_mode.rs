
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferIntentCreateMode {
    #[serde(rename = "PAYMENT")]
    Payment,
    #[serde(rename = "DISBURSEMENT")]
    Disbursement,
}