
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningHit {
    pub analysis: Option<ScreeningHitAnalysis>,
    pub data: Option<ScreeningHitData>,
    pub first_active: String,
    pub historical_since: Option<String>,
    pub id: String,
    pub inactive_since: Option<String>,
    pub list_code: String,
    pub plaid_uid: String,
    pub review_status: String,
    pub source_uid: Option<String>,
}
impl std::fmt::Display for WatchlistScreeningHit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}