
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentConsentPeriodicInterval {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "MONTH")]
    Month,
    #[serde(rename = "YEAR")]
    Year,
}