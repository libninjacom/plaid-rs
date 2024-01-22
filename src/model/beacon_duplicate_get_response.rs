use serde::{Serialize, Deserialize};
use super::{BeaconMatchSummaryAnalysis, BeaconUserRevision};
///A Beacon Duplicate represents a pair of matching Beacon Users and an analysis of the fields they matched on.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconDuplicateGetResponse {
    ///Analysis of which fields matched between one Beacon User and another.
    pub analysis: BeaconMatchSummaryAnalysis,
    ///A Beacon User Revision identifies a Beacon User at some point in its revision history.
    pub beacon_user1: BeaconUserRevision,
    ///A Beacon User Revision identifies a Beacon User at some point in its revision history.
    pub beacon_user2: BeaconUserRevision,
    ///ID of the associated Beacon Duplicate.
    pub id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BeaconDuplicateGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}