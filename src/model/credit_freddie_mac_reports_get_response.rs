
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacVerificationOfAssetsDeal;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacReportsGetResponse {
    #[serde(rename = "DEAL")]
    pub deal: CreditFreddieMacVerificationOfAssetsDeal,
    #[serde(rename = "SchemaVersion")]
    pub schema_version: f64,
    pub request_id: String,
}
impl std::fmt::Display for CreditFreddieMacReportsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}