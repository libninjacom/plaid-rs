
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferQuestionnaireCreateResponse {
    pub onboarding_url: String,
    pub request_id: String,
}
impl std::fmt::Display for TransferQuestionnaireCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}