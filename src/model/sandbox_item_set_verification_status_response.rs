
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxItemSetVerificationStatusResponse {
    pub request_id: String,
}
impl std::fmt::Display for SandboxItemSetVerificationStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}