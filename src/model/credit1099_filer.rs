use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
///An object representing a filer used by 1099-K tax documents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099Filer {
    ///Address on the pay stub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<CreditPayStubAddress>,
    ///Name of filer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Tax identification number of filer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tin: Option<String>,
    ///One of the following values will be provided: Payment Settlement Entity (PSE), Electronic Payment Facilitator (EPF), Other Third Party
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for Credit1099Filer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}