use serde::{Serialize, Deserialize};
use super::SelfieCheckSelfie;
///Additional information for the `selfie_check` step. This field will be `null` unless `steps.selfie_check` has reached a terminal state of either `success` or `failed`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelfieCheck {
    ///An array of selfies submitted to the `selfie_check` step. Each entry represents one user submission.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selfies: Vec<SelfieCheckSelfie>,
    ///The outcome status for the associated Identity Verification attempt's `selfie_check` step. This field will always have the same value as `steps.selfie_check`.
    pub status: String,
}
impl std::fmt::Display for SelfieCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}