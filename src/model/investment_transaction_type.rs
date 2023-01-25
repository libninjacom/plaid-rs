
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentTransactionType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "transfer")]
    Transfer,
}