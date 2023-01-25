
use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099Recipient {
    pub account_number: Option<String>,
    pub address: Option<CreditPayStubAddress>,
    pub facta_filing_requirement: Option<String>,
    pub name: Option<String>,
    pub second_tin_exists: Option<String>,
    pub tin: Option<String>,
}
impl std::fmt::Display for Credit1099Recipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}