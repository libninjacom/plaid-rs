
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningAuditTrail {
    pub dashboard_user_id: Option<String>,
    pub source: String,
    pub timestamp: String,
}
impl std::fmt::Display for WatchlistScreeningAuditTrail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}