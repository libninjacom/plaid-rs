use serde::{Serialize, Deserialize};
use super::LinkSessionSuccessMetadata;
///An object representing an [onSuccess](https://plaid.com/docs/link/web/#onsuccess) callback from Link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionSuccess {
    ///Displayed once a user has successfully linked their Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LinkSessionSuccessMetadata>,
    ///Displayed once a user has successfully linked their Item.
    pub public_token: String,
}
impl std::fmt::Display for LinkSessionSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}