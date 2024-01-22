use serde::{Serialize, Deserialize};
use super::{Loans, Parties, Services};
///An object representing an Asset Report with Freddie Mac schema.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportFreddie {
    ///A collection of loans that are part of a single deal.
    #[serde(rename = "LOANS")]
    pub loans: Loans,
    ///A collection of objects that define specific parties to a deal. This includes the direct participating parties, such as borrower and seller and the indirect parties such as the credit report provider.
    #[serde(rename = "PARTIES")]
    pub parties: Parties,
    ///A collection of objects that describe requests and responses for services.
    #[serde(rename = "SERVICES")]
    pub services: Services,
}
impl std::fmt::Display for AssetReportFreddie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}