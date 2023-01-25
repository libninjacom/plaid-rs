
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "STATUSES")]
    pub statuses: Statuses,
    #[serde(rename = "VERIFICATION_OF_ASSET")]
    pub verification_of_asset: VerificationOfAsset,
}
impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}