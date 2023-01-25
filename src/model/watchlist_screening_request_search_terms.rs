
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningRequestSearchTerms {
    pub country: Option<String>,
    pub date_of_birth: Option<String>,
    pub document_number: Option<String>,
    pub legal_name: String,
    pub watchlist_program_id: String,
}
impl std::fmt::Display for WatchlistScreeningRequestSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}