
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub activity: String,
    pub id: String,
    pub initiated_date: String,
    pub initiator: String,
    pub scopes: ScopesNullable,
    pub state: String,
    pub target_application_id: Option<String>,
}
impl std::fmt::Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}