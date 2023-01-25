
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityProgramGetResponse {
    pub audit_trail: WatchlistScreeningAuditTrail,
    pub created_at: String,
    pub id: String,
    pub is_archived: bool,
    pub is_rescanning_enabled: bool,
    pub lists_enabled: Vec<String>,
    pub name: String,
    pub name_sensitivity: String,
    pub request_id: String,
}
impl std::fmt::Display for WatchlistScreeningEntityProgramGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}