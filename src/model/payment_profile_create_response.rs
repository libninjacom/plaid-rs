use serde::{Serialize, Deserialize};
///PaymentProfileCreateResponse defines the response schema for `/payment_profile/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentProfileCreateResponse {
    ///A payment profile token associated with the Payment Profile data that is being requested.
    pub payment_profile_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentProfileCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}