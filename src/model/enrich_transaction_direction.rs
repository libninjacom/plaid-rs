
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum EnrichTransactionDirection {
    #[serde(rename = "INFLOW")]
    Inflow,
    #[serde(rename = "OUTFLOW")]
    Outflow,
}