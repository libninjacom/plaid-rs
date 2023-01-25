
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchRequest {
    pub country_codes: Vec<String>,
    pub options: Option<InstitutionsSearchRequestOptions>,
    pub products: Option<Vec<String>>,
    pub query: String,
}
impl std::fmt::Display for InstitutionsSearchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}