
use serde::{Serialize, Deserialize};
use super::{
    CreditFreddieMacLoansVoa24, CreditFreddieMacPartiesVoa24,
    CreditFreddieMacServicesVoa24,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetsDealVoa24 {
    #[serde(rename = "LOANS")]
    pub loans: CreditFreddieMacLoansVoa24,
    #[serde(rename = "PARTIES")]
    pub parties: CreditFreddieMacPartiesVoa24,
    #[serde(rename = "SERVICES")]
    pub services: CreditFreddieMacServicesVoa24,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetsDealVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}