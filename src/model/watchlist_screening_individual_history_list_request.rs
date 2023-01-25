
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualHistoryListRequest {
    pub cursor: Option<String>,
    pub watchlist_screening_id: String,
}
impl std::fmt::Display for WatchlistScreeningIndividualHistoryListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}