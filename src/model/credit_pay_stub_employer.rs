use serde::{Serialize, Deserialize};
use super::CreditPayStubAddress;
///Information about the employer on the pay stub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayStubEmployer {
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    ///The name of the employer on the pay stub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for CreditPayStubEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}