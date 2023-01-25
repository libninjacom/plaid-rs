
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningEntityUpdateRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub entity_watchlist_screening_id: String,
    pub reset_fields: Option<Vec<String>>,
    pub search_terms: Option<UpdateEntityScreeningRequestSearchTerms>,
    pub status: Option<String>,
}
impl std::fmt::Display for WatchlistScreeningEntityUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}