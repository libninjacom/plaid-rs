
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacVerificationOfAssetsDealVoa24;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacVerificationOfAssetsVoa24 {
    #[serde(rename = "DEAL")]
    pub deal: Option<CreditFreddieMacVerificationOfAssetsDealVoa24>,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetsVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}