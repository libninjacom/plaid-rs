use serde::{Serialize, Deserialize};
use super::{LinkSessionExit, LinkSessionSuccess};
///An object containing information about a link session. This field will only be present if your client is enabled for Hosted Link (beta). Session data will be provided for up to six hours after the session has ended.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenGetSessionsResponse {
    ///The timestamp at which the link session was finished, if available, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    ///The unique ID for the link session.
    pub link_session_id: String,
    ///An object representing an [onExit](https://plaid.com/docs/link/web/#onexit) callback from Link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<LinkSessionExit>,
    ///An object representing an [onSuccess](https://plaid.com/docs/link/web/#onsuccess) callback from Link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_success: Option<LinkSessionSuccess>,
    ///The timestamp at which the link session was first started, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for LinkTokenGetSessionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}