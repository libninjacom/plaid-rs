use serde::{Serialize, Deserialize};
use super::{CreditSessionError, CreditSessionResults};
///Metadata and results for a Link session
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSession {
    ///The set of errors that occurred during the Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<CreditSessionError>>,
    ///The unique identifier associated with the Link session. This identifier matches the `link_session_id` returned in the onSuccess/onExit callbacks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    ///The set of results for a Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<CreditSessionResults>,
    ///The time when the Link session started
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_start_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for CreditSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}