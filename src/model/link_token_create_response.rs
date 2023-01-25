
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateResponse {
    pub expiration: String,
    pub link_token: String,
    pub request_id: String,
}
impl std::fmt::Display for LinkTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}