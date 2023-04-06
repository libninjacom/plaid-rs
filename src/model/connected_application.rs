
use serde::{Serialize, Deserialize};
use super::Scopes;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedApplication {
    pub application_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_url: Option<String>,
    pub created_at: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_for_access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Scopes>,
}
impl std::fmt::Display for ConnectedApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}