use serde::{Serialize, Deserialize};
use super::{WatchlistScreeningAuditTrail, WatchlistScreeningSearchTerms};
///The screening object allows you to represent a customer in your system, update their profile, and search for them on various watchlists. Note: Rejected customers will not receive new hits, regardless of program configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividual {
    ///ID of the associated user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///A unique ID that identifies the end user in your system. This ID can also be used to associate user-specific data from other Plaid products. Financial Account Matching requires this field and the `/link/token/create` `client_user_id` to be consistent. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    ///ID of the associated screening.
    pub id: String,
    ///Search terms for creating an individual watchlist screening
    pub search_terms: WatchlistScreeningSearchTerms,
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: String,
}
impl std::fmt::Display for WatchlistScreeningIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}