
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieVerificationOfEmploymentVoe25 {
    #[serde(rename = "DEAL")]
    pub deal: Option<CreditFreddieVerificationOfEmploymentDealVoe25>,
}
impl std::fmt::Display for CreditFreddieVerificationOfEmploymentVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}