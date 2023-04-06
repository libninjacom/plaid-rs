
use serde::{Serialize, Deserialize};
use super::WatchlistScreeningHit;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualHitListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub request_id: String,
    pub watchlist_screening_hits: Vec<WatchlistScreeningHit>,
}
impl std::fmt::Display for WatchlistScreeningIndividualHitListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}