use serde::{Serialize, Deserialize};
///An optional object to filter /identity/match results
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityMatchRequestOptions {
    ///An array of `account_ids` to perform fuzzy match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for IdentityMatchRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}