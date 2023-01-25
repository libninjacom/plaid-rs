
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacServicesVoa24 {
    #[serde(rename = "SERVICE")]
    pub service: CreditFreddieMacServiceVoa24,
}
impl std::fmt::Display for CreditFreddieMacServicesVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}