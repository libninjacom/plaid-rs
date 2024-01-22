use serde::{Serialize, Deserialize};
use super::CreditFreddieMacAsset;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacAssets {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub asset: Vec<CreditFreddieMacAsset>,
}
impl std::fmt::Display for CreditFreddieMacAssets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}