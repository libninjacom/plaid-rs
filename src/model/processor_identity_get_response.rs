
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorIdentityGetResponse {
    pub account: AccountIdentity,
    pub request_id: String,
}
impl std::fmt::Display for ProcessorIdentityGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}