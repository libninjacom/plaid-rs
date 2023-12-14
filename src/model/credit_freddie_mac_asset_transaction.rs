
use serde::{Serialize, Deserialize};
use super::{AssetTransactionDescription, AssetTransactionDetail};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacAssetTransaction {
    #[serde(rename = "ASSET_TRANSACTION_DESCRIPTION")]
    pub asset_transaction_description: Vec<AssetTransactionDescription>,
    #[serde(rename = "ASSET_TRANSACTION_DETAIL")]
    pub asset_transaction_detail: AssetTransactionDetail,
}
impl std::fmt::Display for CreditFreddieMacAssetTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}