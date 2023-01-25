
use serde::{Serialize, Deserialize};
use super::{
    AssetHolder, AssetOwners, CreditFreddieMacAssetDetailVoe25,
    CreditFreddieMacAssetTransactionsVoe25,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacAssetVoe25 {
    #[serde(rename = "ASSET_DETAIL")]
    pub asset_detail: CreditFreddieMacAssetDetailVoe25,
    #[serde(rename = "ASSET_HOLDER")]
    pub asset_holder: AssetHolder,
    #[serde(rename = "ASSET_OWNERS")]
    pub asset_owners: AssetOwners,
    #[serde(rename = "ASSET_TRANSACTIONS")]
    pub asset_transactions: CreditFreddieMacAssetTransactionsVoe25,
}
impl std::fmt::Display for CreditFreddieMacAssetVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}