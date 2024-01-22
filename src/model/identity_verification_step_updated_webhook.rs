use serde::{Serialize, Deserialize};
///Fired when an end user has completed a step of the Identity Verification process.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationStepUpdatedWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: String,
    ///`STEP_UPDATED`
    pub webhook_code: String,
    ///`IDENTITY_VERIFICATION`
    pub webhook_type: String,
}
impl std::fmt::Display for IdentityVerificationStepUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}