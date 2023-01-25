
use serde::{Serialize, Deserialize};
use super::{Parties, Services, Loans};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportFreddie {
    #[serde(rename = "LOANS")]
    pub loans: Loans,
    #[serde(rename = "PARTIES")]
    pub parties: Parties,
    #[serde(rename = "SERVICES")]
    pub services: Services,
}
impl std::fmt::Display for AssetReportFreddie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}