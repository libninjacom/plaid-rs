
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualCreateRequest {
    pub client_user_id: Option<String>,
    pub search_terms: WatchlistScreeningRequestSearchTerms,
}
impl std::fmt::Display for WatchlistScreeningIndividualCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}