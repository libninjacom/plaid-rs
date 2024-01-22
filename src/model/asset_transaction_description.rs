use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetTransactionDescription {
    ///Asset Transaction Description String up to 3 occurances 1 required.
    #[serde(rename = "AssetTransactionDescription")]
    pub asset_transaction_description: String,
}
impl std::fmt::Display for AssetTransactionDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}