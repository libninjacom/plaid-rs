
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportSyndicationOriginalReport {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub fraud_date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for BeaconReportSyndicationOriginalReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}