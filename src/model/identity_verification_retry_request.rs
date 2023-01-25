
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationRetryRequest {
    pub client_user_id: String,
    pub steps: Option<IdentityVerificationRetryRequestStepsObject>,
    pub strategy: String,
    pub template_id: String,
}
impl std::fmt::Display for IdentityVerificationRetryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}