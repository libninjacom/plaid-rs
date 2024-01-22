use serde::{Serialize, Deserialize};
use super::{CreditFreddieMacPartyIndividual, Roles, TaxpayerIdentifiers};
///A collection of information about a single party to a transaction. Included direct participants like the borrower and seller as well as indirect participants such as the flood certificate provider.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacParty {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "INDIVIDUAL")]
    pub individual: CreditFreddieMacPartyIndividual,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ROLES")]
    pub roles: Roles,
    ///The collection of TAXPAYER_IDENTIFICATION elements
    #[serde(rename = "TAXPAYER_IDENTIFIERS")]
    pub taxpayer_identifiers: TaxpayerIdentifiers,
}
impl std::fmt::Display for CreditFreddieMacParty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}