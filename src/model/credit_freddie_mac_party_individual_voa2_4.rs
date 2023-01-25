
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacPartyIndividualVoa24 {
    #[serde(rename = "NAME")]
    pub name: CreditFreddieMacIndividualNameVoa24,
}
impl std::fmt::Display for CreditFreddieMacPartyIndividualVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}