
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckRequest {
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub payroll_institution: Option<IncomeVerificationPrecheckPayrollInstitution>,
    pub transactions_access_token: Option<String>,
    pub transactions_access_tokens: Option<Vec<String>>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
    pub user: Option<IncomeVerificationPrecheckUser>,
}
impl std::fmt::Display for IncomeVerificationPrecheckRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}