
use serde::{Serialize, Deserialize};
use super::{EntityWatchlistScreeningSearchTerms, WatchlistScreeningAuditTrail};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityWatchlistScreening {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    pub id: String,
    pub search_terms: EntityWatchlistScreeningSearchTerms,
    pub status: String,
}
impl std::fmt::Display for EntityWatchlistScreening {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}