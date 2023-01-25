
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacAssetsVoa24;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetResponseVoa24 {
    #[serde(rename = "ASSETS")]
    pub assets: CreditFreddieMacAssetsVoa24,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetResponseVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}