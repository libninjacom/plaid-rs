
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningEntityHitListRequest {
    pub cursor: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl std::fmt::Display for WatchlistScreeningEntityHitListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}