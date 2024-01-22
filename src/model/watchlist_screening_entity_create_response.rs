use serde::{Serialize, Deserialize};
use super::{EntityWatchlistScreeningSearchTerms, WatchlistScreeningAuditTrail};
///The entity screening object allows you to represent an entity in your system, update its profile, and search for it on various watchlists. Note: Rejected entity screenings will not receive new hits, regardless of entity program configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningEntityCreateResponse {
    ///ID of the associated user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///A unique ID that identifies the end user in your system. This ID can also be used to associate user-specific data from other Plaid products. Financial Account Matching requires this field and the `/link/token/create` `client_user_id` to be consistent. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    ///ID of the associated entity screening.
    pub id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Search terms associated with an entity used for searching against watchlists
    pub search_terms: EntityWatchlistScreeningSearchTerms,
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: String,
}
impl std::fmt::Display for WatchlistScreeningEntityCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}