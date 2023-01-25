
use serde::{Serialize, Deserialize};
use super::{
    CreditSessionBankIncomeResult, CreditSessionDocumentIncomeResult,
    CreditSessionItemAddResult, CreditSessionPayrollIncomeResult,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionResults {
    pub bank_income_results: Option<Vec<CreditSessionBankIncomeResult>>,
    pub document_income_results: Option<CreditSessionDocumentIncomeResult>,
    pub item_add_results: Option<Vec<CreditSessionItemAddResult>>,
    pub payroll_income_results: Option<Vec<CreditSessionPayrollIncomeResult>>,
}
impl std::fmt::Display for CreditSessionResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}