
use serde::{Serialize, Deserialize};
use super::{CreditFreddieMacVerificationOfAssetVoa24, Statuses};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacServiceVoa24 {
    #[serde(rename = "STATUSES")]
    pub statuses: Statuses,
    #[serde(rename = "VERIFICATION_OF_ASSET")]
    pub verification_of_asset: Vec<CreditFreddieMacVerificationOfAssetVoa24>,
}
impl std::fmt::Display for CreditFreddieMacServiceVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}