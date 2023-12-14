
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacAsset;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssets {
    #[serde(rename = "ASSET")]
    pub asset: Vec<CreditFreddieMacAsset>,
}
impl std::fmt::Display for CreditFreddieMacAssets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}