
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncidentUpdate {
    pub description: Option<String>,
    pub status: Option<String>,
    pub updated_date: Option<String>,
}
impl std::fmt::Display for IncidentUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}