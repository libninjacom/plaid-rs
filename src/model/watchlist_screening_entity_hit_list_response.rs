use serde::{Serialize, Deserialize};
use super::EntityWatchlistScreeningHit;
///Paginated list of entity watchlist screening hits
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningEntityHitListResponse {
    ///List of entity watchlist screening hits
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_watchlist_screening_hits: Vec<EntityWatchlistScreeningHit>,
    ///An identifier that determines which page of results you receive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WatchlistScreeningEntityHitListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}