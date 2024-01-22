use serde::{Serialize, Deserialize};
use super::AssetReportFreddie;
///AssetReportFreddieGetResponse defines the response schema for `/asset_report/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportFreddieGetResponse {
    ///An object representing an Asset Report with Freddie Mac schema.
    #[serde(rename = "DEAL")]
    pub deal: AssetReportFreddie,
    ///The Verification Of Assets (aka VOA or Freddie Mac Schema) schema version.
    #[serde(rename = "SchemaVersion")]
    pub schema_version: f64,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportFreddieGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}