
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetTransactionDescription {
    #[serde(rename = "AssetTransactionDescription")]
    pub asset_transaction_description: String,
}
impl std::fmt::Display for AssetTransactionDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}