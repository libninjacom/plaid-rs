use serde::{Serialize, Deserialize};
use super::CreditFreddieMacAssets;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacVerificationOfAssetResponse {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSETS")]
    pub assets: CreditFreddieMacAssets,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}