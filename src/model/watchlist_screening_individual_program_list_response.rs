
use serde::{Serialize, Deserialize};
use super::IndividualWatchlistProgram;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualProgramListResponse {
    pub next_cursor: Option<String>,
    pub request_id: String,
    pub watchlist_programs: Vec<IndividualWatchlistProgram>,
}
impl std::fmt::Display for WatchlistScreeningIndividualProgramListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}