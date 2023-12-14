
use serde::{Serialize, Deserialize};
use super::{BeaconAuditTrail, FraudAmount};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportCreateResponse {
    pub audit_trail: BeaconAuditTrail,
    pub beacon_user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_amount: Option<FraudAmount>,
    pub fraud_date: chrono::NaiveDate,
    pub id: String,
    pub request_id: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for BeaconReportCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}