
use serde::{Serialize, Deserialize};
use super::SandboxPublicTokenCreateRequestIncomeVerificationBankIncome;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestOptionsIncomeVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<SandboxPublicTokenCreateRequestIncomeVerificationBankIncome>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_source_types: Option<Vec<String>>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptionsIncomeVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}