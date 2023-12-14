
use serde::{Serialize, Deserialize};
use super::{
    LinkTokenCreateRequestIncomeVerificationBankIncome,
    LinkTokenCreateRequestIncomeVerificationPayrollIncome,
    LinkTokenCreateRequestUserStatedIncomeSource,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIncomeVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_report_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<LinkTokenCreateRequestIncomeVerificationBankIncome>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_source_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_verification_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payroll_income: Option<LinkTokenCreateRequestIncomeVerificationPayrollIncome>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stated_income_sources: Option<Vec<LinkTokenCreateRequestUserStatedIncomeSource>>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}