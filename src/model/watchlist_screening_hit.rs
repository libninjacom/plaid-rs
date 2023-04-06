
use serde::{Serialize, Deserialize};
use super::{ScreeningHitAnalysis, ScreeningHitData};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningHit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<ScreeningHitAnalysis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ScreeningHitData>,
    pub first_active: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_since: Option<chrono::DateTime<chrono::Utc>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_since: Option<chrono::DateTime<chrono::Utc>>,
    pub list_code: String,
    pub plaid_uid: String,
    pub review_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_uid: Option<String>,
}
impl std::fmt::Display for WatchlistScreeningHit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}