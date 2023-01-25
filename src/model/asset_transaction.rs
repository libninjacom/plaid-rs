
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTransaction {
    #[serde(rename = "ASSET_TRANSACTION_DESCRIPTON")]
    pub asset_transaction_descripton: Vec<AssetTransactionDescription>,
    #[serde(rename = "ASSET_TRANSACTION_DETAIL")]
    pub asset_transaction_detail: AssetTransactionDetail,
}
impl std::fmt::Display for AssetTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}