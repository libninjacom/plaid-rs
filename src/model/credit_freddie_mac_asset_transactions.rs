
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacAssetTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssetTransactions {
    #[serde(rename = "ASSET_TRANSACTION")]
    pub asset_transaction: Vec<CreditFreddieMacAssetTransaction>,
}
impl std::fmt::Display for CreditFreddieMacAssetTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}