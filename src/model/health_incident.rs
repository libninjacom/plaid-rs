
use serde::{Serialize, Deserialize};
use super::IncidentUpdate;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIncident {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub incident_updates: Vec<IncidentUpdate>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub title: String,
}
impl std::fmt::Display for HealthIncident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}