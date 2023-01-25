
use serde::{Serialize, Deserialize};
use super::{AssetTransactionDescription, CreditFreddieMacAssetTransactionDetailVoe25};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacAssetTransactionVoe25 {
    #[serde(rename = "ASSET_TRANSACTION_DESCRIPTION")]
    pub asset_transaction_description: Vec<AssetTransactionDescription>,
    #[serde(rename = "ASSET_TRANSACTION_DETAIL")]
    pub asset_transaction_detail: CreditFreddieMacAssetTransactionDetailVoe25,
}
impl std::fmt::Display for CreditFreddieMacAssetTransactionVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}