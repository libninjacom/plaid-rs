
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityCreateRequest {
    pub client_user_id: Option<String>,
    pub search_terms: EntityWatchlistSearchTerms,
}
impl std::fmt::Display for WatchlistScreeningEntityCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}