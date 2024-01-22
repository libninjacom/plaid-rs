use serde::{Serialize, Deserialize};
///Search inputs for creating an entity watchlist screening
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityWatchlistSearchTerms {
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The numeric or alphanumeric identifier associated with this document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    ///A valid email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: String,
    ///The name of the organization being screened.
    pub legal_name: String,
    ///A phone number in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    ///An 'http' or 'https' URL (must begin with either of those).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for EntityWatchlistSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}