
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationRetryRequestStepsObject {
    pub documentary_verification: bool,
    pub kyc_check: bool,
    pub selfie_check: bool,
    pub verify_sms: bool,
}
impl std::fmt::Display for IdentityVerificationRetryRequestStepsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}