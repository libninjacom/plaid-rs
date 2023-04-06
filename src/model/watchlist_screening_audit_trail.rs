
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningAuditTrail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_user_id: Option<String>,
    pub source: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for WatchlistScreeningAuditTrail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}