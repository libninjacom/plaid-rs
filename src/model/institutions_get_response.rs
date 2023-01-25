
use serde::{Serialize, Deserialize};
use super::Institution;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetResponse {
    pub institutions: Vec<Institution>,
    pub request_id: String,
    pub total: i64,
}
impl std::fmt::Display for InstitutionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}