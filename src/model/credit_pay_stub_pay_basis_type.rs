
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditPayStubPayBasisType {
    #[serde(rename = "SALARY")]
    Salary,
    #[serde(rename = "HOURLY")]
    Hourly,
    #[serde(rename = "COMMISSION")]
    Commission,
}