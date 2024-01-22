use serde::{Serialize, Deserialize};
///LinkDeliveryCreateResponse defines the response schema for `/link_delivery/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryCreateResponse {
    ///The ID for the Hosted Link session. Same as the `link_token` string excluding the "link-{env}-" prefix.
    pub link_delivery_session_id: String,
    ///The URL to the Hosted Link session, which will be delivered by the specified delivery method.
    pub link_delivery_url: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LinkDeliveryCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}