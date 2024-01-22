use serde::{Serialize, Deserialize};
use super::{PartyIndividual, Roles, TaxpayerIdentifiers};
///A collection of information about a single party to a transaction. Included direct participants like the borrower and seller as well as indirect participants such as the flood certificate provider.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Party {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "INDIVIDUAL")]
    pub individual: PartyIndividual,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ROLES")]
    pub roles: Roles,
    ///The collection of TAXPAYER_IDENTIFICATION elements
    #[serde(rename = "TAXPAYER_IDENTIFIERS")]
    pub taxpayer_identifiers: TaxpayerIdentifiers,
}
impl std::fmt::Display for Party {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}