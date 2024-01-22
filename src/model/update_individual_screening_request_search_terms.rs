use serde::{Serialize, Deserialize};
///Search terms for editing an individual watchlist screening
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateIndividualScreeningRequestSearchTerms {
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///The numeric or alphanumeric identifier associated with this document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    ///The legal name of the individual being screened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    ///ID of the associated program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchlist_program_id: Option<String>,
}
impl std::fmt::Display for UpdateIndividualScreeningRequestSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}