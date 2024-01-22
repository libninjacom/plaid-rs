use serde::{Serialize, Deserialize};
use super::{AssetReport, Warning};
///AssetReportGetResponse defines the response schema for `/asset_report/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportGetResponse {
    ///An object representing an Asset Report
    pub report: AssetReport,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///If the Asset Report generation was successful but identity information cannot be returned, this array will contain information about the errors causing identity information to be missing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Warning>,
}
impl std::fmt::Display for AssetReportGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}