
use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099Payer {
    pub address: Option<CreditPayStubAddress>,
    pub name: Option<String>,
    pub telephone_number: Option<String>,
    pub tin: Option<String>,
}
impl std::fmt::Display for Credit1099Payer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}