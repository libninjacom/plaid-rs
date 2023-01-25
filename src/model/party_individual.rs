
use serde::{Serialize, Deserialize};
use super::IndividualName;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyIndividual {
    #[serde(rename = "NAME")]
    pub name: IndividualName,
}
impl std::fmt::Display for PartyIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}