
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateIndividualScreeningRequestSearchTerms {
    pub country: Option<String>,
    pub date_of_birth: Option<String>,
    pub document_number: Option<String>,
    pub legal_name: Option<String>,
    pub watchlist_program_id: Option<String>,
}
impl std::fmt::Display for UpdateIndividualScreeningRequestSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}