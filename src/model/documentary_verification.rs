use serde::{Serialize, Deserialize};
use super::DocumentaryVerificationDocument;
///Data, images, analysis, and results from the `documentary_verification` step. This field will be `null` unless `steps.documentary_verification` has reached a terminal state of either `success` or `failed`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentaryVerification {
    /**An array of documents submitted to the `documentary_verification` step. Each entry represents one user submission, where each submission will contain both a front and back image, or just a front image, depending on the document type.

Note: Plaid will automatically let a user submit a new set of document images up to three times if we detect that a previous attempt might have failed due to user error. For example, if the first set of document images are blurry or obscured by glare, the user will be asked to capture their documents again, resulting in at least two separate entries within `documents`. If the overall `documentary_verification` is `failed`, the user has exhausted their retry attempts.*/
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documents: Vec<DocumentaryVerificationDocument>,
    ///The outcome status for the associated Identity Verification attempt's `documentary_verification` step. This field will always have the same value as `steps.documentary_verification`.
    pub status: String,
}
impl std::fmt::Display for DocumentaryVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}