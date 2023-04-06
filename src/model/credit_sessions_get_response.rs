
use serde::{Serialize, Deserialize};
use super::CreditSession;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionsGetResponse {
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<CreditSession>>,
}
impl std::fmt::Display for CreditSessionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}