
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomePrecheckRequest {
    pub access_tokens: Option<Vec<String>>,
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub payroll_institution: Option<IncomeVerificationPrecheckPayrollInstitution>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
    pub user_token: Option<String>,
}
impl std::fmt::Display for CreditPayrollIncomePrecheckRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}