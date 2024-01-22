use serde::{Serialize, Deserialize};
///LinkDeliveryGetRequest defines the response schema for `/link_delivery/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryGetResponse {
    ///An array of access tokens associated with the Hosted Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Vec<String>>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Hosted Link session was completed at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Hosted Link session was created at.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///An array of `item_id`s associated with the Hosted Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**The status of the given Hosted Link session.

`CREATED`: The session is created but not yet accessed by the user

`OPENED`: The session is opened by the user but not yet completed

`EXITED`: The session has been exited by the user

`COMPLETED`: The session has been completed by the user

`EXPIRED`: The session has expired*/
    pub status: String,
}
impl std::fmt::Display for LinkDeliveryGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}