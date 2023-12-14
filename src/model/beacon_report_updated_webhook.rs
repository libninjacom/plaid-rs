
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportUpdatedWebhook {
    pub beacon_report_id: String,
    pub environment: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for BeaconReportUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}