
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationCreateRequest {
    pub gave_consent: bool,
    pub is_idempotent: Option<bool>,
    pub is_shareable: bool,
    pub template_id: String,
    pub user: IdentityVerificationRequestUser,
}
impl std::fmt::Display for IdentityVerificationCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}