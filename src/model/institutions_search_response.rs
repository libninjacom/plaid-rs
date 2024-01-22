use serde::{Serialize, Deserialize};
use super::Institution;
///InstitutionsSearchResponse defines the response schema for `/institutions/search`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchResponse {
    ///An array of institutions matching the search criteria
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub institutions: Vec<Institution>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for InstitutionsSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}