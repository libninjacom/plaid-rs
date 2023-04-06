
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenCreateResponse {
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub link_token: String,
    pub request_id: String,
}
impl std::fmt::Display for LinkTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}