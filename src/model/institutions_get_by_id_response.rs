
use serde::{Serialize, Deserialize};
use super::Institution;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsGetByIdResponse {
    pub institution: Institution,
    pub request_id: String,
}
impl std::fmt::Display for InstitutionsGetByIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}