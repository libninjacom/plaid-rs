
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacIndividualName;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacPartyIndividual {
    #[serde(rename = "NAME")]
    pub name: CreditFreddieMacIndividualName,
}
impl std::fmt::Display for CreditFreddieMacPartyIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}