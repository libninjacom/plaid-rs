
use serde::{Serialize, Deserialize};
use super::WatchlistScreeningReview;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualReviewListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub request_id: String,
    pub watchlist_screening_reviews: Vec<WatchlistScreeningReview>,
}
impl std::fmt::Display for WatchlistScreeningIndividualReviewListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}