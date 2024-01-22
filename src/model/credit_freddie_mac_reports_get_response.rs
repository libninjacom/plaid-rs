use serde::{Serialize, Deserialize};
use super::CreditFreddieMacVerificationOfAssetsDeal;
///CreditFreddieMacReportsGetResponse defines the response schema for `/credit/freddie_mac/reports/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacReportsGetResponse {
    ///An object representing an Asset Report with Freddie Mac schema.
    #[serde(rename = "DEAL")]
    pub deal: CreditFreddieMacVerificationOfAssetsDeal,
    ///The Verification Of Assets (VOA) schema version.
    #[serde(rename = "SchemaVersion")]
    pub schema_version: f64,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditFreddieMacReportsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}