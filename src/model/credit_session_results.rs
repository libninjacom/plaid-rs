use serde::{Serialize, Deserialize};
use super::{
    CreditSessionBankEmploymentResult, CreditSessionBankIncomeResult,
    CreditSessionDocumentIncomeResult, CreditSessionItemAddResult,
    CreditSessionPayrollIncomeResult,
};
///The set of results for a Link session.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionResults {
    ///The set of bank employment verifications for the Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_employment_results: Option<Vec<CreditSessionBankEmploymentResult>>,
    ///The set of bank income verifications for the Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income_results: Option<Vec<CreditSessionBankIncomeResult>>,
    ///The details of a document income verification in Link
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_income_results: Option<CreditSessionDocumentIncomeResult>,
    ///The set of Item adds for the Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_add_results: Option<Vec<CreditSessionItemAddResult>>,
    ///The set of payroll income verifications for the Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_income_results: Option<Vec<CreditSessionPayrollIncomeResult>>,
}
impl std::fmt::Display for CreditSessionResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}