use serde::{Serialize, Deserialize};
///AssetReportCreateResponse defines the response schema for `/asset_report/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportCreateResponse {
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: String,
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}