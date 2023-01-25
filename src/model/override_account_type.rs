
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum OverrideAccountType {
    #[serde(rename = "investment")]
    Investment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "depository")]
    Depository,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "payroll")]
    Payroll,
    #[serde(rename = "other")]
    Other,
}