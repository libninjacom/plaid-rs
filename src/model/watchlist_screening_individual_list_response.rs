
use serde::{Serialize, Deserialize};
use super::WatchlistScreeningIndividual;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub request_id: String,
    pub watchlist_screenings: Vec<WatchlistScreeningIndividual>,
}
impl std::fmt::Display for WatchlistScreeningIndividualListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}