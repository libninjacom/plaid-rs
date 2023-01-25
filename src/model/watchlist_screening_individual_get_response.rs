
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualGetResponse {
    pub assignee: Option<String>,
    pub audit_trail: WatchlistScreeningAuditTrail,
    pub client_user_id: Option<String>,
    pub id: String,
    pub request_id: String,
    pub search_terms: WatchlistScreeningSearchTerms,
    pub status: String,
}
impl std::fmt::Display for WatchlistScreeningIndividualGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}