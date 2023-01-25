
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatedIncomeSourceFrequency {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "SEMI_MONTHLY")]
    SemiMonthly,
    #[serde(rename = "MONTHLY")]
    Monthly,
}