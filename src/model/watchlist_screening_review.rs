use serde::{Serialize, Deserialize};
use super::WatchlistScreeningAuditTrail;
/**A review submitted by a team member for an individual watchlist screening. A review can be either a comment on the current screening state, actions taken
against hits attached to the watchlist screening, or both.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningReview {
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub confirmed_hits: Vec<String>,
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dismissed_hits: Vec<String>,
    ///ID of the associated review.
    pub id: String,
}
impl std::fmt::Display for WatchlistScreeningReview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}