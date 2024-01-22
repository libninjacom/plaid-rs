use serde::{Serialize, Deserialize};
use super::WatchlistScreeningReview;
///Paginated list of screening reviews
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualReviewListResponse {
    ///An identifier that determines which page of results you receive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///List of screening reviews
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub watchlist_screening_reviews: Vec<WatchlistScreeningReview>,
}
impl std::fmt::Display for WatchlistScreeningIndividualReviewListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}