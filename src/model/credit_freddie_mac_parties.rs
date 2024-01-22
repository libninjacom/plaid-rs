use serde::{Serialize, Deserialize};
use super::CreditFreddieMacParty;
///A collection of objects that define specific parties to a deal. This includes the direct participating parties, such as borrower and seller and the indirect parties such as the credit report provider.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacParties {
    #[serde(rename = "PARTY")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party: Vec<CreditFreddieMacParty>,
}
impl std::fmt::Display for CreditFreddieMacParties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}