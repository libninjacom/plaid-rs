use serde::{Serialize, Deserialize};
///AssetReportRemoveResponse defines the response schema for `/asset_report/remove`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportRemoveResponse {
    ///`true` if the Asset Report was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}