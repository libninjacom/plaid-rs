
use serde::{Serialize, Deserialize};
use super::{
    CreditFreddieMacLoansVoa24, CreditFreddieMacServicesVoe25,
    CreditFreddieMacPartiesVoa24,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieVerificationOfEmploymentDealVoe25 {
    #[serde(rename = "LOANS")]
    pub loans: CreditFreddieMacLoansVoa24,
    #[serde(rename = "PARTIES")]
    pub parties: CreditFreddieMacPartiesVoa24,
    #[serde(rename = "SERVICES")]
    pub services: CreditFreddieMacServicesVoe25,
}
impl std::fmt::Display for CreditFreddieVerificationOfEmploymentDealVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}