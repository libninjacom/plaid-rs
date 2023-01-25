
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacServiceVoe25;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacServicesVoe25 {
    #[serde(rename = "SERVICE")]
    pub service: CreditFreddieMacServiceVoe25,
}
impl std::fmt::Display for CreditFreddieMacServicesVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}