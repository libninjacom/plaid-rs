
use serde::{Serialize, Deserialize};
use super::{BaseReport, BaseReportWarning};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReportGetResponse {
    pub report: BaseReport,
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<BaseReportWarning>>,
}
impl std::fmt::Display for BaseReportGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}