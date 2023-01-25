
use serde::{Serialize, Deserialize};
use super::AssetOwner;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetOwners {
    #[serde(rename = "ASSET_OWNER")]
    pub asset_owner: Vec<AssetOwner>,
}
impl std::fmt::Display for AssetOwners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}