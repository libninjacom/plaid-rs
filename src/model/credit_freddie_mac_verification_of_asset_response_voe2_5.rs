
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetResponseVoe25 {
    #[serde(rename = "ASSETS")]
    pub assets: CreditFreddieMacAssetsVoe25,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetResponseVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}