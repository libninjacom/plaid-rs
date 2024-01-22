use serde::{Serialize, Deserialize};
///LinkTokenCreateResponse defines the response schema for `/link/token/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateResponse {
    ///The expiration date for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. A `link_token` created to generate a `public_token` that will be exchanged for a new `access_token` expires after 4 hours. A `link_token` created for an existing Item (such as when updating an existing `access_token` by launching Link in update mode) expires after 30 minutes.
    pub expiration: chrono::DateTime<chrono::Utc>,
    ///A URL of a Plaid-hosted Link flow that will use the Link token returned by this request. Only present if the client is enabled for Hosted Link (beta).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosted_link_url: Option<String>,
    ///A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    pub link_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LinkTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}