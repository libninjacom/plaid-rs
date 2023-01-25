
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacPartyVoa24 {
    #[serde(rename = "INDIVIDUAL")]
    pub individual: CreditFreddieMacPartyIndividualVoa24,
    #[serde(rename = "ROLES")]
    pub roles: Roles,
    #[serde(rename = "TAXPAYER_IDENTIFIERS")]
    pub taxpayer_identifiers: TaxpayerIdentifiers,
}
impl std::fmt::Display for CreditFreddieMacPartyVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}