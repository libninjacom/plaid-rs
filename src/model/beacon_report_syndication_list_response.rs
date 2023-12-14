
use serde::{Serialize, Deserialize};
use super::BeaconReportSyndication;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportSyndicationListResponse {
    pub beacon_report_syndications: Vec<BeaconReportSyndication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub request_id: String,
}
impl std::fmt::Display for BeaconReportSyndicationListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}