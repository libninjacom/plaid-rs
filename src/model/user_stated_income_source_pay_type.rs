
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatedIncomeSourcePayType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "GROSS")]
    Gross,
    #[serde(rename = "NET")]
    Net,
}