use serde::{Serialize, Deserialize};
use super::Institution;
///InstitutionsGetResponse defines the response schema for `/institutions/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetResponse {
    ///A list of Plaid institutions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub institutions: Vec<Institution>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///The total number of institutions available via this endpoint
    pub total: i64,
}
impl std::fmt::Display for InstitutionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}