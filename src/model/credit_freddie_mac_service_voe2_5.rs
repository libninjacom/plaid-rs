
use serde::{Serialize, Deserialize};
use super::{CreditFreddieMacVerificationOfAssetVoe25, Statuses};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacServiceVoe25 {
    #[serde(rename = "STATUSES")]
    pub statuses: Statuses,
    #[serde(rename = "VERIFICATION_OF_ASSET")]
    pub verification_of_asset: Vec<CreditFreddieMacVerificationOfAssetVoe25>,
}
impl std::fmt::Display for CreditFreddieMacServiceVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}