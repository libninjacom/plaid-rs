
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationOfAssetResponse {
    #[serde(rename = "ASSETS")]
    pub assets: Assets,
}
impl std::fmt::Display for VerificationOfAssetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}