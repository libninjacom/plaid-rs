use serde::{Serialize, Deserialize};
///SandboxPaymentProfileResetLoginResponse defines the response schema for `/sandbox/payment_profile/reset_login`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPaymentProfileResetLoginResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///`true` if the call succeeded
    pub reset_login: bool,
}
impl std::fmt::Display for SandboxPaymentProfileResetLoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}