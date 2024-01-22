use serde::{Serialize, Deserialize};
///UserCreateResponse defines the response schema for `/user/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    pub user_id: String,
    ///The user token associated with the User data is being requested for.
    pub user_token: String,
}
impl std::fmt::Display for UserCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}