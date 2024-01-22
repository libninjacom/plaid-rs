use serde::{Serialize, Deserialize};
///Fired when identity verification has been retried, which can be triggered via the dashboard or the API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationRetriedWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: String,
    ///`RETRIED`
    pub webhook_code: String,
    ///`IDENTITY_VERIFICATION`
    pub webhook_type: String,
}
impl std::fmt::Display for IdentityVerificationRetriedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}