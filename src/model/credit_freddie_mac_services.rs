
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacService;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacServices {
    #[serde(rename = "SERVICE")]
    pub service: CreditFreddieMacService,
}
impl std::fmt::Display for CreditFreddieMacServices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}