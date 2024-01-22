use serde::{Serialize, Deserialize};
use super::{AssetTransactionDescription, AssetTransactionDetail};
///An object representing...
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssetTransaction {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET_TRANSACTION_DESCRIPTION")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub asset_transaction_description: Vec<AssetTransactionDescription>,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET_TRANSACTION_DETAIL")]
    pub asset_transaction_detail: AssetTransactionDetail,
}
impl std::fmt::Display for CreditFreddieMacAssetTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}