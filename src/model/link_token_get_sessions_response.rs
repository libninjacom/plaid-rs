
use serde::{Serialize, Deserialize};
use super::{LinkSessionExit, LinkSessionSuccess};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenGetSessionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub link_session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<LinkSessionExit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success: Option<LinkSessionSuccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for LinkTokenGetSessionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}