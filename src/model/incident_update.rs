use serde::{Serialize, Deserialize};
///An update on the health incident
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncidentUpdate {
    ///The content of the update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The status of the incident.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///The date when the update was published, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for IncidentUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}