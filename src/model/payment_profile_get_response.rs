
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProfileGetResponse {
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub request_id: String,
    pub status: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for PaymentProfileGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}