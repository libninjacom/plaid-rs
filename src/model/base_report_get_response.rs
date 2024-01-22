use serde::{Serialize, Deserialize};
use super::{BaseReport, BaseReportWarning};
///BaseReportGetResponse defines the response schema for `/cra/base_report/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportGetResponse {
    ///An object representing a Base Report
    pub report: BaseReport,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///If the Base Report generation was successful but identity information cannot be returned, this array will contain information about the errors causing identity information to be missing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<BaseReportWarning>>,
}
impl std::fmt::Display for BaseReportGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}