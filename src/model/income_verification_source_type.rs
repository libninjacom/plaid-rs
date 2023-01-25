
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeVerificationSourceType {
    #[serde(rename = "bank")]
    Bank,
    #[serde(rename = "payroll")]
    Payroll,
}