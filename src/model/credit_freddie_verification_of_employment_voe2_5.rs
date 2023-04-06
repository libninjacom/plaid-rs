
use serde::{Serialize, Deserialize};
use super::CreditFreddieVerificationOfEmploymentDealVoe25;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieVerificationOfEmploymentVoe25 {
    #[serde(rename = "DEAL")]
    pub deal: CreditFreddieVerificationOfEmploymentDealVoe25,
    #[serde(rename = "SchemaVersion")]
    pub schema_version: f64,
}
impl std::fmt::Display for CreditFreddieVerificationOfEmploymentVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}