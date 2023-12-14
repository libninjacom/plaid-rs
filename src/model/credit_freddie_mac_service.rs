
use serde::{Serialize, Deserialize};
use super::{CreditFreddieMacVerificationOfAsset, Statuses};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacService {
    #[serde(rename = "STATUSES")]
    pub statuses: Statuses,
    #[serde(rename = "VERIFICATION_OF_ASSET")]
    pub verification_of_asset: Vec<CreditFreddieMacVerificationOfAsset>,
}
impl std::fmt::Display for CreditFreddieMacService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}