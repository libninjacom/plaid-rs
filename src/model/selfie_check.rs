
use serde::{Serialize, Deserialize};
use super::SelfieCheckSelfie;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelfieCheck {
    pub selfies: Vec<SelfieCheckSelfie>,
    pub status: String,
}
impl std::fmt::Display for SelfieCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}