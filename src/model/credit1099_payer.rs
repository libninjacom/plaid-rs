use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
///An object representing a payer used by 1099-MISC tax documents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099Payer {
    ///Address on the pay stub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<CreditPayStubAddress>,
    ///Name of payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Telephone number of payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone_number: Option<String>,
    ///Tax identification number of payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tin: Option<String>,
}
impl std::fmt::Display for Credit1099Payer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}