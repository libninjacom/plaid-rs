
use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099Filer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreditPayStubAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tin: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for Credit1099Filer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}