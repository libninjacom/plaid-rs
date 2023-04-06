
use serde::{Serialize, Deserialize};
use super::{CreditSessionError, CreditSessionResults};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<CreditSessionError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<CreditSessionResults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_start_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for CreditSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}