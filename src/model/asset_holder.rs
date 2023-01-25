
use serde::{Serialize, Deserialize};
use super::AssetHolderName;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetHolder {
    #[serde(rename = "NAME")]
    pub name: AssetHolderName,
}
impl std::fmt::Display for AssetHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}