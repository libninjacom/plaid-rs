
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualUpdateRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub reset_fields: Option<Vec<String>>,
    pub search_terms: Option<UpdateIndividualScreeningRequestSearchTerms>,
    pub status: Option<String>,
    pub watchlist_screening_id: String,
}
impl std::fmt::Display for WatchlistScreeningIndividualUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}