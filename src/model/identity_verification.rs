use serde::{Serialize, Deserialize};
use super::{
    DocumentaryVerification, IdentityVerificationStepSummary,
    IdentityVerificationTemplateReference, IdentityVerificationUserData, KycCheckDetails,
    RiskCheckDetails, SelfieCheck,
};
///A identity verification attempt represents a customer's attempt to verify their identity, reflecting the required steps for completing the session, the results for each step, and information collected in the process.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerification {
    ///A unique ID that identifies the end user in your system. This ID can also be used to associate user-specific data from other Plaid products. Financial Account Matching requires this field and the `/link/token/create` `client_user_id` to be consistent. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    pub client_user_id: String,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    ///An ISO8601 formatted timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///Data, images, analysis, and results from the `documentary_verification` step. This field will be `null` unless `steps.documentary_verification` has reached a terminal state of either `success` or `failed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentary_verification: Option<DocumentaryVerification>,
    ///ID of the associated Identity Verification attempt.
    pub id: String,
    ///Additional information for the `kyc_check` step. This field will be `null` unless `steps.kyc_check` has reached a terminal state of either `success` or `failed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kyc_check: Option<KycCheckDetails>,
    ///The ID for the Identity Verification preceding this session. This field will only be filled if the current Identity Verification is a retry of a previous attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_attempt_id: Option<String>,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redacted_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Additional information for the `risk_check` step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_check: Option<RiskCheckDetails>,
    ///Additional information for the `selfie_check` step. This field will be `null` unless `steps.selfie_check` has reached a terminal state of either `success` or `failed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selfie_check: Option<SelfieCheck>,
    ///A shareable URL that can be sent directly to the user to complete verification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shareable_url: Option<String>,
    /**The status of this Identity Verification attempt.


`active` - The Identity Verification attempt is incomplete. The user may have completed part of the session, but has neither failed or passed.

`success` - The Identity Verification attempt has completed, passing all steps defined to the associated Identity Verification template

`failed` - The user failed one or more steps in the session and was told to contact support.

`expired` - The Identity Verification attempt was active for a long period of time without being completed and was automatically marked as expired. Note that sessions currently do not expire. Automatic expiration is expected to be enabled in the future.

`canceled` - The Identity Verification attempt was canceled, either via the dashboard by a user, or via API. The user may have completed part of the session, but has neither failed or passed.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.*/
    pub status: String,
    /**Each step will be one of the following values:


`active` - This step is the user's current step. They are either in the process of completing this step, or they recently closed their Identity Verification attempt while in the middle of this step. Only one step will be marked as `active` at any given point.

`success` - The Identity Verification attempt has completed this step.

`failed` - The user failed this step. This can either call the user to fail the session as a whole, or cause them to fallback to another step depending on how the Identity Verification template is configured. A failed step does not imply a failed session.

`waiting_for_prerequisite` - The user needs to complete another step first, before they progress to this step. This step may never run, depending on if the user fails an earlier step or if the step is only run as a fallback.

`not_applicable` - This step will not be run for this session.

`skipped` - The retry instructions that created this Identity Verification attempt specified that this step should be skipped.

`expired` - This step had not yet been completed when the Identity Verification attempt as a whole expired.

`canceled` - The Identity Verification attempt was canceled before the user completed this step.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.

`manually_approved` - The step was manually overridden to pass by a team member in the dashboard.

`manually_rejected` - The step was manually overridden to fail by a team member in the dashboard.*/
    pub steps: IdentityVerificationStepSummary,
    ///The resource ID and version number of the template configuring the behavior of a given Identity Verification.
    pub template: IdentityVerificationTemplateReference,
    ///The identity data that was either collected from the user or provided via API in order to perform an Identity Verification.
    pub user: IdentityVerificationUserData,
    ///ID of the associated screening.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchlist_screening_id: Option<String>,
}
impl std::fmt::Display for IdentityVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}