use serde::{Serialize, Deserialize};
use super::EntityWatchlistScreeningReview;
///Paginated list of entity watchlist screening reviews
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningEntityReviewListResponse {
    ///List of entity watchlist screening reviews
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_watchlist_screening_reviews: Vec<EntityWatchlistScreeningReview>,
    ///An identifier that determines which page of results you receive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WatchlistScreeningEntityReviewListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}