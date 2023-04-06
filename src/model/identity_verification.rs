
use serde::{Serialize, Deserialize};
use super::{
    DocumentaryVerification, IdentityVerificationStepSummary,
    IdentityVerificationTemplateReference, IdentityVerificationUserData, KycCheckDetails,
    RiskCheckDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerification {
    pub client_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentary_verification: Option<DocumentaryVerification>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kyc_check: Option<KycCheckDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attempt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_check: Option<RiskCheckDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareable_url: Option<String>,
    pub status: String,
    pub steps: IdentityVerificationStepSummary,
    pub template: IdentityVerificationTemplateReference,
    pub user: IdentityVerificationUserData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchlist_screening_id: Option<String>,
}
impl std::fmt::Display for IdentityVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}