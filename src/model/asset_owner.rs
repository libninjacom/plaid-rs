use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetOwner {
    ///Account Owner Full Name.
    #[serde(rename = "AssetOwnerText")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_owner_text: Option<String>,
}
impl std::fmt::Display for AssetOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}