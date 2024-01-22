use serde::{Serialize, Deserialize};
use super::IncomeVerificationPrecheckEmployerAddress;
///Information about the end user's employer
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckEmployer {
    ///The address of the employer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<IncomeVerificationPrecheckEmployerAddress>,
    ///The employer's name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The employer's tax id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    ///The URL for the employer's public website
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}