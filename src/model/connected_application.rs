
use serde::{Serialize, Deserialize};
use super::Scopes;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectedApplication {
    pub application_id: String,
    pub application_url: Option<String>,
    pub created_at: String,
    pub display_name: Option<String>,
    pub logo_url: Option<String>,
    pub name: String,
    pub reason_for_access: Option<String>,
    pub scopes: Option<Scopes>,
}
impl std::fmt::Display for ConnectedApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}