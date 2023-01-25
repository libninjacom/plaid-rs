
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssetTransactionsVoa24 {
    #[serde(rename = "ASSET_TRANSACTION")]
    pub asset_transaction: Vec<CreditFreddieMacAssetTransactionVoa24>,
}
impl std::fmt::Display for CreditFreddieMacAssetTransactionsVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}