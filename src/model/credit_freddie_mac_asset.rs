
use serde::{Serialize, Deserialize};
use super::{
    AssetDetail, AssetHolder, AssetOwners, CreditFreddieMacAssetTransactions,
    ValidationSources,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacAsset {
    #[serde(rename = "ASSET_DETAIL")]
    pub asset_detail: AssetDetail,
    #[serde(rename = "ASSET_HOLDER")]
    pub asset_holder: AssetHolder,
    #[serde(rename = "ASSET_OWNERS")]
    pub asset_owners: AssetOwners,
    #[serde(rename = "ASSET_TRANSACTIONS")]
    pub asset_transactions: CreditFreddieMacAssetTransactions,
    #[serde(rename = "VALIDATION_SOURCES")]
    pub validation_sources: ValidationSources,
}
impl std::fmt::Display for CreditFreddieMacAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}