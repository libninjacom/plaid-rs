
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUserGetResponse {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email_address: String,
    pub id: String,
    pub request_id: String,
    pub status: String,
}
impl std::fmt::Display for DashboardUserGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}