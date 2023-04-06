
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatusInvestments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_update: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_update: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for ItemStatusInvestments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}