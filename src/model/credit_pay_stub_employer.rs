
use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayStubEmployer {
    pub address: CreditPayStubAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for CreditPayStubEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}