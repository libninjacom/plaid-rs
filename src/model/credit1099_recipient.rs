
use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099Recipient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreditPayStubAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facta_filing_requirement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_tin_exists: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tin: Option<String>,
}
impl std::fmt::Display for Credit1099Recipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}