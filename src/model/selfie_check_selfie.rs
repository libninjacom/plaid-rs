use serde::{Serialize, Deserialize};
use super::{SelfieAnalysis, SelfieCapture};
///Captures and analysis from a user's selfie.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelfieCheckSelfie {
    ///High level descriptions of how the associated selfie was processed. If a selfie fails verification, the details in the `analysis` object should help clarify why the selfie was rejected.
    pub analysis: SelfieAnalysis,
    ///The `attempt` field begins with 1 and increments with each subsequent selfie upload.
    pub attempt: i64,
    ///The image or video capture of a selfie. Only one of image or video URL will be populated per selfie.
    pub capture: SelfieCapture,
    ///An outcome status for this specific selfie. Distinct from the overall `selfie_check.status` that summarizes the verification outcome from one or more selfies.
    pub status: String,
}
impl std::fmt::Display for SelfieCheckSelfie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}