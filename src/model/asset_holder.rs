use serde::{Serialize, Deserialize};
use super::AssetHolderName;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetHolder {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "NAME")]
    pub name: AssetHolderName,
}
impl std::fmt::Display for AssetHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}