
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacPartiesVoa24 {
    #[serde(rename = "PARTY")]
    pub party: Vec<CreditFreddieMacPartyVoa24>,
}
impl std::fmt::Display for CreditFreddieMacPartiesVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}