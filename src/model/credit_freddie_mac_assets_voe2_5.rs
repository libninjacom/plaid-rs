
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacAssetVoe25;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssetsVoe25 {
    #[serde(rename = "ASSET")]
    pub asset: Vec<CreditFreddieMacAssetVoe25>,
}
impl std::fmt::Display for CreditFreddieMacAssetsVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}