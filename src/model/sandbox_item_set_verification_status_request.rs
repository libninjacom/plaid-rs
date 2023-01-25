
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxItemSetVerificationStatusRequest {
    pub access_token: String,
    pub account_id: String,
    pub verification_status: String,
}
impl std::fmt::Display for SandboxItemSetVerificationStatusRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}