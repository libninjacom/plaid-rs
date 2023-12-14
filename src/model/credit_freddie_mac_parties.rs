
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacParty;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacParties {
    #[serde(rename = "PARTY")]
    pub party: Vec<CreditFreddieMacParty>,
}
impl std::fmt::Display for CreditFreddieMacParties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}