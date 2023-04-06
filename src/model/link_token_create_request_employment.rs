
use serde::{Serialize, Deserialize};
use super::LinkTokenCreateRequestEmploymentBankIncome;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestEmployment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_employment: Option<LinkTokenCreateRequestEmploymentBankIncome>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_source_types: Option<Vec<String>>,
}
impl std::fmt::Display for LinkTokenCreateRequestEmployment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}