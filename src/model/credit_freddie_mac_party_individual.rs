use serde::{Serialize, Deserialize};
use super::CreditFreddieMacIndividualName;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacPartyIndividual {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "NAME")]
    pub name: CreditFreddieMacIndividualName,
}
impl std::fmt::Display for CreditFreddieMacPartyIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}