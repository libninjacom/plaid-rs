
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemAccessTokenInvalidateResponse {
    pub new_access_token: String,
    pub request_id: String,
}
impl std::fmt::Display for ItemAccessTokenInvalidateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}