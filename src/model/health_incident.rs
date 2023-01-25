
use serde::{Serialize, Deserialize};
use super::IncidentUpdate;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthIncident {
    pub end_date: Option<String>,
    pub incident_updates: Vec<IncidentUpdate>,
    pub start_date: String,
    pub title: String,
}
impl std::fmt::Display for HealthIncident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}