use serde::{Serialize, Deserialize};
use super::AssetReportUser;
///An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportCreateRequestOptions {
    ///Use this field to request a `fast_asset` report. When Fast Assets is requested, Plaid will create two versions of the Asset Report: first, the Fast Asset Report, which will contain only current identity and balance information, and later, the Full Asset Report, which will also contain historical balance information and transaction data. A `PRODUCT_READY` webhook will be fired for each Asset Report when it is ready, and the `report_type` field will indicate whether the webhook is firing for the `full` or `fast` Asset Report. To retrieve the Fast Asset Report, call `/asset_report/get` with `fast_report` set to `true`. There is no additional charge for using Fast Assets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<String>>,
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<String>,
    ///true to return balance and identity earlier as a fast report. Defaults to false if omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_fast_report: Option<bool>,
    ///Additional information that can be included in the asset report. Possible values: `"investments"`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<AssetReportUser>,
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for AssetReportCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}