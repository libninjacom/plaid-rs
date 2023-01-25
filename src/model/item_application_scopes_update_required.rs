
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateRequired {
    pub access_token: String,
    pub application_id: String,
    pub context: String,
    pub scopes: Scopes,
}
impl std::fmt::Display for ItemApplicationScopesUpdateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}