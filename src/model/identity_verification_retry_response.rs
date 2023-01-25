
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationRetryResponse {
    pub client_user_id: String,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub documentary_verification: Option<DocumentaryVerification>,
    pub id: String,
    pub kyc_check: Option<KycCheckDetails>,
    pub previous_attempt_id: Option<String>,
    pub request_id: String,
    pub shareable_url: Option<String>,
    pub status: String,
    pub steps: IdentityVerificationStepSummary,
    pub template: IdentityVerificationTemplateReference,
    pub user: IdentityVerificationUserData,
    pub watchlist_screening_id: Option<String>,
}
impl std::fmt::Display for IdentityVerificationRetryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}