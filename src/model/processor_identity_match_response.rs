
use serde::{Serialize, Deserialize};
use super::AccountIdentityMatchScore;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorIdentityMatchResponse {
    pub account: AccountIdentityMatchScore,
    pub request_id: String,
}
impl std::fmt::Display for ProcessorIdentityMatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}