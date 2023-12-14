
use serde::{Serialize, Deserialize};
use super::{CreditFreddieMacPartyIndividual, Roles, TaxpayerIdentifiers};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacParty {
    #[serde(rename = "INDIVIDUAL")]
    pub individual: CreditFreddieMacPartyIndividual,
    #[serde(rename = "ROLES")]
    pub roles: Roles,
    #[serde(rename = "TAXPAYER_IDENTIFIERS")]
    pub taxpayer_identifiers: TaxpayerIdentifiers,
}
impl std::fmt::Display for CreditFreddieMacParty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}