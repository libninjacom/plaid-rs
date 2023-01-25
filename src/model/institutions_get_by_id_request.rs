
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetByIdRequest {
    pub country_codes: Vec<String>,
    pub institution_id: String,
    pub options: Option<InstitutionsGetByIdRequestOptions>,
}
impl std::fmt::Display for InstitutionsGetByIdRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}