
use serde::{Serialize, Deserialize};
use super::BeaconReport;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportListResponse {
    pub beacon_reports: Vec<BeaconReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub request_id: String,
}
impl std::fmt::Display for BeaconReportListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}