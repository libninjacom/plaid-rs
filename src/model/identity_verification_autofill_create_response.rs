use serde::{Serialize, Deserialize};
use super::IdentityVerificationAutofillUserData;
///Autofill represents unverified customer information. This needs to be confirmed by the customer before using.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationAutofillCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///A status enum indicating whether autofill succeeded or failed.
    pub status: String,
    ///User information that was autofilled. All this information should be confirmed by the user before using.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<IdentityVerificationAutofillUserData>,
}
impl std::fmt::Display for IdentityVerificationAutofillCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}