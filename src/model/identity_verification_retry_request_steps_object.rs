use serde::{Serialize, Deserialize};
/**Instructions for the `custom` retry strategy specifying which steps should be required or skipped.


Note:


This field must be provided when the retry strategy is `custom` and must be omitted otherwise.

Custom retries override settings in your Plaid Template. For example, if your Plaid Template has `verify_sms` disabled, a custom retry with `verify_sms` enabled will still require the step.

The `selfie_check` step is currently not supported on the sandbox server. Sandbox requests will silently disable the `selfie_check` step when provided.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationRetryRequestStepsObject {
    ///A boolean field specifying whether the new session should require or skip the `documentary_verification` step.
    pub documentary_verification: bool,
    ///A boolean field specifying whether the new session should require or skip the `kyc_check` step.
    pub kyc_check: bool,
    ///A boolean field specifying whether the new session should require or skip the `selfie_check` step.
    pub selfie_check: bool,
    ///A boolean field specifying whether the new session should require or skip the `verify_sms` step.
    pub verify_sms: bool,
}
impl std::fmt::Display for IdentityVerificationRetryRequestStepsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}