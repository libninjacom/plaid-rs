use serde::{Serialize, Deserialize};
///ItemAccessTokenInvalidateResponse defines the response schema for `/item/access_token/invalidate`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemAccessTokenInvalidateResponse {
    ///The access token associated with the Item data is being requested for.
    pub new_access_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemAccessTokenInvalidateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}