
use serde::{Serialize, Deserialize};
use super::{
    LinkTokenCreateRequestIncomeVerificationBankIncome,
    LinkTokenCreateRequestIncomeVerificationPayrollIncome,
    LinkTokenCreateRequestUserStatedIncomeSource,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIncomeVerification {
    pub access_tokens: Option<Vec<String>>,
    pub asset_report_id: Option<String>,
    pub bank_income: Option<LinkTokenCreateRequestIncomeVerificationBankIncome>,
    pub income_source_types: Option<Vec<String>>,
    pub income_verification_id: Option<String>,
    pub payroll_income: Option<LinkTokenCreateRequestIncomeVerificationPayrollIncome>,
    pub precheck_id: Option<String>,
    pub stated_income_sources: Option<Vec<LinkTokenCreateRequestUserStatedIncomeSource>>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}