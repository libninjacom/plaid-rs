
use serde::{Serialize, Deserialize};
use super::EntityWatchlistScreeningHit;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningEntityHitListResponse {
    pub entity_watchlist_screening_hits: Vec<EntityWatchlistScreeningHit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub request_id: String,
}
impl std::fmt::Display for WatchlistScreeningEntityHitListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}