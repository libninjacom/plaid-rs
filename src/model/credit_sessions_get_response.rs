use serde::{Serialize, Deserialize};
use super::CreditSession;
///CreditSessionsGetResponse defines the response schema for `/credit/sessions/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionsGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///A list of Link sessions for the user. Sessions will be sorted in reverse chronological order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<CreditSession>>,
}
impl std::fmt::Display for CreditSessionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}