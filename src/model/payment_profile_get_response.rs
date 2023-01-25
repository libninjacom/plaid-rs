
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentProfileGetResponse {
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub request_id: String,
    pub status: String,
    pub updated_at: String,
}
impl std::fmt::Display for PaymentProfileGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}