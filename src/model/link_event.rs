use serde::{Serialize, Deserialize};
///An event that occurred while the user was going through Link
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkEvent {
    ///UUID that can be used to deduplicate events
    pub event_id: String,
    ///Event name
    pub event_name: String,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub timestamp: String,
}
impl std::fmt::Display for LinkEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}