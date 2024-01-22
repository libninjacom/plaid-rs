use serde::{Serialize, Deserialize};
///A Beacon User Revision identifies a Beacon User at some point in its revision history.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserRevision {
    ///ID of the associated Beacon User.
    pub id: String,
    ///The `version` field begins with 1 and increments with each subsequent revision.
    pub version: i64,
}
impl std::fmt::Display for BeaconUserRevision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}