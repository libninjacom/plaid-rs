use serde::{Serialize, Deserialize};
use super::JwkPublicKey;
///WebhookVerificationKeyGetResponse defines the response schema for `/webhook_verification_key/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebhookVerificationKeyGetResponse {
    ///A JSON Web Key (JWK) that can be used in conjunction with [JWT libraries](https://jwt.io/#libraries-io) to verify Plaid webhooks
    pub key: JwkPublicKey,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WebhookVerificationKeyGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}