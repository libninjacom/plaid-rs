
use serde::{Serialize, Deserialize};
use super::IncomeVerificationPrecheckEmployerAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckEmployer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<IncomeVerificationPrecheckEmployerAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}