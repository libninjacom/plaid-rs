
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeVerificationPayrollFlowType {
    #[serde(rename = "payroll_digital_income")]
    PayrollDigitalIncome,
    #[serde(rename = "payroll_document_income")]
    PayrollDocumentIncome,
}