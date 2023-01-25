
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateRequest {
    pub access_token: String,
    pub application_id: String,
    pub context: String,
    pub scopes: Scopes,
    pub state: Option<String>,
}
impl std::fmt::Display for ItemApplicationScopesUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}