
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityMatchRequest {
    pub access_token: String,
    pub options: Option<IdentityMatchRequestOptions>,
    pub user: Option<IdentityMatchUser>,
}
impl std::fmt::Display for IdentityMatchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}