use serde::{Serialize, Deserialize};
use super::{LinkTokenGetMetadataResponse, LinkTokenGetSessionsResponse};
///LinkTokenGetResponse defines the response schema for `/link/token/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenGetResponse {
    ///The creation timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///The expiration timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    ///Information about Link sessions created using this `link_token`. This field will only be present if your client is enabled for Hosted Link (beta). Session data will be provided for up to six hours after the session has ended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_sessions: Option<Vec<LinkTokenGetSessionsResponse>>,
    ///A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    pub link_token: String,
    ///An object specifying the arguments originally provided to the `/link/token/create` call.
    pub metadata: LinkTokenGetMetadataResponse,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LinkTokenGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}