use serde::{Serialize, Deserialize};
use super::{CreditFreddieMacLoans, CreditFreddieMacParties, CreditFreddieMacServices};
///An object representing an Asset Report with Freddie Mac schema.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacVerificationOfAssetsDeal {
    ///A collection of loans that are part of a single deal.
    #[serde(rename = "LOANS")]
    pub loans: CreditFreddieMacLoans,
    ///A collection of objects that define specific parties to a deal. This includes the direct participating parties, such as borrower and seller and the indirect parties such as the credit report provider.
    #[serde(rename = "PARTIES")]
    pub parties: CreditFreddieMacParties,
    ///A collection of objects that describe requests and responses for services.
    #[serde(rename = "SERVICES")]
    pub services: CreditFreddieMacServices,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetsDeal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}