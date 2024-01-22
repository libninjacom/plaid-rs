use serde::{Serialize, Deserialize};
///Specifies option for initializing Link for use with the Identity Verification product.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIdentityVerification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent: Option<bool>,
    /**A flag specifying whether the end user has already agreed to a privacy policy specifying that their data will be shared with Plaid for verification purposes.

If `gave_consent` is set to `true`, the `accept_tos` step will be marked as `skipped` and the end user's session will start at the next step requirement.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gave_consent: Option<bool>,
    pub template_id: String,
}
impl std::fmt::Display for LinkTokenCreateRequestIdentityVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}