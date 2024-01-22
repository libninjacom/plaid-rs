use serde::{Serialize, Deserialize};
///PaymentProfileGetResponse defines the response schema for `/payment_profile/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentProfileGetResponse {
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Payment Profile was created at
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Payment Profile was deleted at. Always `null` if the Payment Profile has not been deleted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**The status of the given Payment Profile.

`READY`: This Payment Profile is ready to be used to create transfers using `/transfer/authorization/create` and `/transfer/create`.

`PENDING`: This Payment Profile is not ready to be used. You’ll need to call `/link/token/create` and provide the `payment_profile_token` in the `transfer.payment_profile_token` field to initiate the account linking experience.

`REMOVED`: This Payment Profile has been removed.*/
    pub status: String,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time the given Payment Profile was updated at
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for PaymentProfileGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}