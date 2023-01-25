
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferQuestionnaireCreateRequest {
    pub originator_client_id: String,
    pub redirect_uri: String,
}
impl std::fmt::Display for TransferQuestionnaireCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}