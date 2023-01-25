
use serde::{Serialize, Deserialize};
use super::{AssetDetail, AssetHolder, AssetOwners, AssetTransactions, ValidationSources};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "ASSET_DETAIL")]
    pub asset_detail: AssetDetail,
    #[serde(rename = "ASSET_HOLDER")]
    pub asset_holder: AssetHolder,
    #[serde(rename = "ASSET_OWNERS")]
    pub asset_owners: AssetOwners,
    #[serde(rename = "ASSET_TRANSACTIONS")]
    pub asset_transactions: AssetTransactions,
    #[serde(rename = "VALIDATION_SOURCES")]
    pub validation_sources: ValidationSources,
}
impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}