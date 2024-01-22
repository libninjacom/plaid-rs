use serde::{Serialize, Deserialize};
use super::AssetReportUser;
///An optional object to filter `/asset_report/refresh` results. If provided, cannot be `null`. If not specified, the `options` from the original call to `/asset_report/create` will be used.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportRefreshRequestOptions {
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<String>,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<AssetReportUser>,
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for AssetReportRefreshRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}