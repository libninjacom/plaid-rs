
use serde::{Serialize, Deserialize};
use super::Scopes;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub activity: String,
    pub id: String,
    pub initiated_date: chrono::NaiveDate,
    pub initiator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Scopes>,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_application_id: Option<String>,
}
impl std::fmt::Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}