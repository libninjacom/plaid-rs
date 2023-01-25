
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationStepSummary {
    pub accept_tos: String,
    pub documentary_verification: String,
    pub kyc_check: String,
    pub risk_check: String,
    pub selfie_check: String,
    pub verify_sms: String,
    pub watchlist_screening: String,
}
impl std::fmt::Display for IdentityVerificationStepSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}