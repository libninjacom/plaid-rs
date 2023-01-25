
use serde::{Serialize, Deserialize};
use super::{PartyIndividual, Roles, TaxpayerIdentifiers};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    #[serde(rename = "INDIVIDUAL")]
    pub individual: PartyIndividual,
    #[serde(rename = "ROLES")]
    pub roles: Roles,
    #[serde(rename = "TAXPAYER_IDENTIFIERS")]
    pub taxpayer_identifiers: TaxpayerIdentifiers,
}
impl std::fmt::Display for Party {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}