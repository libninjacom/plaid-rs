
use serde::{Serialize, Deserialize};
use super::SandboxPublicTokenCreateRequestIncomeVerificationBankIncome;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestOptionsIncomeVerification {
    pub bank_income: Option<SandboxPublicTokenCreateRequestIncomeVerificationBankIncome>,
    pub income_source_types: Option<Vec<String>>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptionsIncomeVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}