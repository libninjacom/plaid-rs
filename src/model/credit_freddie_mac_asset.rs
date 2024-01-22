use serde::{Serialize, Deserialize};
use super::{
    AssetDetail, AssetHolder, AssetOwners, CreditFreddieMacAssetTransactions,
    ValidationSources,
};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAsset {
    ///Details about an asset.
    #[serde(rename = "ASSET_DETAIL")]
    pub asset_detail: AssetDetail,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET_HOLDER")]
    pub asset_holder: AssetHolder,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET_OWNERS")]
    pub asset_owners: AssetOwners,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET_TRANSACTIONS")]
    pub asset_transactions: CreditFreddieMacAssetTransactions,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "VALIDATION_SOURCES")]
    pub validation_sources: ValidationSources,
}
impl std::fmt::Display for CreditFreddieMacAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}