
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssetTransactionsVoe25 {
    #[serde(rename = "ASSET_TRANSACTION")]
    pub asset_transaction: Vec<CreditFreddieMacAssetTransactionVoe25>,
}
impl std::fmt::Display for CreditFreddieMacAssetTransactionsVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}