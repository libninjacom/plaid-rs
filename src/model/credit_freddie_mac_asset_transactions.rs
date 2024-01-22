use serde::{Serialize, Deserialize};
use super::CreditFreddieMacAssetTransaction;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssetTransactions {
    #[serde(rename = "ASSET_TRANSACTION")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub asset_transaction: Vec<CreditFreddieMacAssetTransaction>,
}
impl std::fmt::Display for CreditFreddieMacAssetTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}