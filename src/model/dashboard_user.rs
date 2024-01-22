use serde::{Serialize, Deserialize};
///Account information associated with a team member with access to the Plaid dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardUser {
    ///An ISO8601 formatted timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///A valid email address.
    pub email_address: String,
    ///ID of the associated user.
    pub id: String,
    ///The current status of the user.
    pub status: String,
}
impl std::fmt::Display for DashboardUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}