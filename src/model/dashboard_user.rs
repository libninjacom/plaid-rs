
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardUser {
    pub created_at: String,
    pub email_address: String,
    pub id: String,
    pub status: String,
}
impl std::fmt::Display for DashboardUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}