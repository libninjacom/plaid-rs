use serde::{Serialize, Deserialize};
use super::{BeaconAuditTrail, BeaconUserData};
///A Beacon User represents an end user that has been scanned against the Beacon Network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserGetResponse {
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: BeaconAuditTrail,
    ///A unique ID that identifies the end user in your system. This ID can also be used to associate user-specific data from other Plaid products. Financial Account Matching requires this field and the `/link/token/create` `client_user_id` to be consistent. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    pub client_user_id: String,
    ///An ISO8601 formatted timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///ID of the associated Beacon User.
    pub id: String,
    ///ID of the associated Beacon Program.
    pub program_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**A status of a Beacon User.

`rejected`: The Beacon User has been rejected for fraud. Users can be automatically or manually rejected.

`pending_review`: The Beacon User has been marked for review.

`cleared`: The Beacon User has been cleared of fraud.*/
    pub status: String,
    ///An ISO8601 formatted timestamp. This field indicates the last time the resource was modified.
    pub updated_at: chrono::DateTime<chrono::Utc>,
    ///A Beacon User's data and resulting analysis when checked against duplicate records and the Beacon Fraud Network.
    pub user: BeaconUserData,
}
impl std::fmt::Display for BeaconUserGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}