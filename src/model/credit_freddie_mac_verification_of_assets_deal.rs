
use serde::{Serialize, Deserialize};
use super::{CreditFreddieMacLoans, CreditFreddieMacParties, CreditFreddieMacServices};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetsDeal {
    #[serde(rename = "LOANS")]
    pub loans: CreditFreddieMacLoans,
    #[serde(rename = "PARTIES")]
    pub parties: CreditFreddieMacParties,
    #[serde(rename = "SERVICES")]
    pub services: CreditFreddieMacServices,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetsDeal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}