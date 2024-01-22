use serde::{Serialize, Deserialize};
use super::AssetOwner;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetOwners {
    ///Multiple Occurances of Account Owners Full Name up to 4.
    #[serde(rename = "ASSET_OWNER")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub asset_owner: Vec<AssetOwner>,
}
impl std::fmt::Display for AssetOwners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}