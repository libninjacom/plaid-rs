
use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099Filer {
    pub address: Option<CreditPayStubAddress>,
    pub name: Option<String>,
    pub tin: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for Credit1099Filer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}