use serde::{Serialize, Deserialize};
use super::Institution;
///InstitutionsGetByIdResponse defines the response schema for `/institutions/get_by_id`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetByIdResponse {
    ///Details relating to a specific financial institution
    pub institution: Institution,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for InstitutionsGetByIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}