
use serde::{Serialize, Deserialize};
use super::AssetTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetTransactions {
    #[serde(rename = "ASSET_TRANSACTION")]
    pub asset_transaction: Vec<AssetTransaction>,
}
impl std::fmt::Display for AssetTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}