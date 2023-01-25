
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityWatchlistSearchTerms {
    pub country: Option<String>,
    pub document_number: Option<String>,
    pub email_address: Option<String>,
    pub entity_watchlist_program_id: String,
    pub legal_name: String,
    pub phone_number: Option<String>,
    pub url: Option<String>,
}
impl std::fmt::Display for EntityWatchlistSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}