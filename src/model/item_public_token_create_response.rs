use serde::{Serialize, Deserialize};
///ItemPublicTokenCreateResponse defines the response schema for `/item/public_token/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemPublicTokenCreateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    ///A `public_token` for the particular Item corresponding to the specified `access_token`
    pub public_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemPublicTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}