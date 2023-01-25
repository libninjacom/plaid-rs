
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssetsVoa24 {
    #[serde(rename = "ASSET")]
    pub asset: Vec<CreditFreddieMacAssetVoa24>,
}
impl std::fmt::Display for CreditFreddieMacAssetsVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}