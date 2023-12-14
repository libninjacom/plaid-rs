
use serde::{Serialize, Deserialize};
use super::{SelfieAnalysis, SelfieCapture};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfieCheckSelfie {
    pub analysis: SelfieAnalysis,
    pub attempt: i64,
    pub capture: SelfieCapture,
    pub status: String,
}
impl std::fmt::Display for SelfieCheckSelfie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}