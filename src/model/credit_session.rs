
use serde::{Serialize, Deserialize};
use super::{CreditSessionError, CreditSessionResults};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSession {
    pub errors: Option<Vec<CreditSessionError>>,
    pub link_session_id: Option<String>,
    pub results: Option<CreditSessionResults>,
    pub session_start_time: Option<String>,
}
impl std::fmt::Display for CreditSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}