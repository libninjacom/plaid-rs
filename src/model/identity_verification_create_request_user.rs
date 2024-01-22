use serde::{Serialize, Deserialize};
use super::{IdentityVerificationRequestUserName, UserAddress, UserIdNumber};
/**User information collected outside of Link, most likely via your own onboarding process.

Each of the following identity fields are optional:

`email_address`

`phone_number`

`date_of_birth`

`name`

`address`

`id_number`

Specifically, these fields are optional in that they can either be fully provided (satisfying every required field in their subschema) or omitted from the request entirely by not providing the key or value.
Providing these fields via the API will result in Link skipping the data collection process for the associated user. All verification steps enabled in the associated Identity Verification Template will still be run. Verification steps will either be run immediately, or once the user completes the `accept_tos` step, depending on the value provided to the `gave_consent` field.
If you are not using the shareable URL feature, you can optionally provide these fields via `/link/token/create` instead; both `/identity_verification/create` and `/link/token/create` are valid ways to provide this information. Note that if you provide a non-`null` user data object via `/identity_verification/create`, any user data fields entered via `/link/token/create` for the same `client_user_id` will be ignored when prefilling Link.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationCreateRequestUser {
    /**Home address for the user. Supported values are: not provided, address with only country code or full address.

For more context on this field, see [Input Validation by Country](https://plaid.com/docs/identity-verification/hybrid-input-validation/#input-validation-by-country).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<UserAddress>,
    ///Specifying `user.client_user_id` is deprecated. Please provide `client_user_id` at the root level instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///A valid email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<UserIdNumber>,
    ///You can use this field to pre-populate the user's legal name; if it is provided here, they will not be prompted to enter their name in the identity verification attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<IdentityVerificationRequestUserName>,
    ///A phone number in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for IdentityVerificationCreateRequestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}