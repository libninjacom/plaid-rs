
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetRequest {
    pub count: i64,
    pub country_codes: Vec<String>,
    pub offset: i64,
    pub options: Option<InstitutionsGetRequestOptions>,
}
impl std::fmt::Display for InstitutionsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}