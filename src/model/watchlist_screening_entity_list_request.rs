
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningEntityListRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub cursor: Option<String>,
    pub entity_watchlist_program_id: String,
    pub status: Option<String>,
}
impl std::fmt::Display for WatchlistScreeningEntityListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}