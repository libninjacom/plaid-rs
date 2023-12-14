
use serde::{Serialize, Deserialize};
use super::{BeaconAuditTrail, BeaconUserData};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserCreateResponse {
    pub audit_trail: BeaconAuditTrail,
    pub client_user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: String,
    pub program_id: String,
    pub request_id: String,
    pub status: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user: BeaconUserData,
}
impl std::fmt::Display for BeaconUserCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}