
use serde::{Serialize, Deserialize};
use super::{
    CreditSessionBankEmploymentResult, CreditSessionBankIncomeResult,
    CreditSessionDocumentIncomeResult, CreditSessionItemAddResult,
    CreditSessionPayrollIncomeResult,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_employment_results: Option<Vec<CreditSessionBankEmploymentResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income_results: Option<Vec<CreditSessionBankIncomeResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_income_results: Option<CreditSessionDocumentIncomeResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_add_results: Option<Vec<CreditSessionItemAddResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payroll_income_results: Option<Vec<CreditSessionPayrollIncomeResult>>,
}
impl std::fmt::Display for CreditSessionResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}