
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityWatchlistScreeningSearchTerms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    pub entity_watchlist_program_id: String,
    pub legal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub version: f64,
}
impl std::fmt::Display for EntityWatchlistScreeningSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}