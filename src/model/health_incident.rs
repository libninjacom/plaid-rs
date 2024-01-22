use serde::{Serialize, Deserialize};
use super::IncidentUpdate;
///A status health incident
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthIncident {
    ///The end date of the incident, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    ///Updates on the health incident.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub incident_updates: Vec<IncidentUpdate>,
    ///The start date of the incident, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub start_date: chrono::DateTime<chrono::Utc>,
    ///The title of the incident
    pub title: String,
}
impl std::fmt::Display for HealthIncident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}