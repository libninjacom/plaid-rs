
use serde::{Serialize, Deserialize};
use super::IncomeVerificationPrecheckEmployerAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckEmployer {
    pub address: Option<IncomeVerificationPrecheckEmployerAddress>,
    pub name: Option<String>,
    pub tax_id: Option<String>,
    pub url: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}