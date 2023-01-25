
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualListRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub cursor: Option<String>,
    pub status: Option<String>,
    pub watchlist_program_id: String,
}
impl std::fmt::Display for WatchlistScreeningIndividualListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}