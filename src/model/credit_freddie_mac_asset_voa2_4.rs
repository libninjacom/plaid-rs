
use serde::{Serialize, Deserialize};
use super::{
    AssetDetail, AssetHolder, AssetOwners, CreditFreddieMacAssetTransactionsVoa24,
    ValidationSources,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacAssetVoa24 {
    #[serde(rename = "ASSET_DETAIL")]
    pub asset_detail: AssetDetail,
    #[serde(rename = "ASSET_HOLDER")]
    pub asset_holder: AssetHolder,
    #[serde(rename = "ASSET_OWNERS")]
    pub asset_owners: AssetOwners,
    #[serde(rename = "ASSET_TRANSACTIONS")]
    pub asset_transactions: CreditFreddieMacAssetTransactionsVoa24,
    #[serde(rename = "VALIDATION_SOURCES")]
    pub validation_sources: ValidationSources,
}
impl std::fmt::Display for CreditFreddieMacAssetVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}