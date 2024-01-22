use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
///An object representing a recipient used in both 1099-K and 1099-MISC tax documents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099Recipient {
    ///Account number number of recipient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    ///Address on the pay stub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<CreditPayStubAddress>,
    ///Checked if FACTA is a filing requirement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facta_filing_requirement: Option<String>,
    ///Name of recipient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Checked if 2nd TIN exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second_tin_exists: Option<String>,
    ///Tax identification number of recipient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tin: Option<String>,
}
impl std::fmt::Display for Credit1099Recipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}