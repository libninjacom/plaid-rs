use serde::{Serialize, Deserialize};
///Fired when the status of an identity verification has been updated, which can be triggered via the dashboard or the API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationStatusUpdatedWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: String,
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    ///`IDENTITY_VERIFICATION`
    pub webhook_type: String,
}
impl std::fmt::Display for IdentityVerificationStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}