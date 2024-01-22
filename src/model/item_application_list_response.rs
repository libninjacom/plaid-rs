use serde::{Serialize, Deserialize};
use super::ConnectedApplication;
///Describes the connected application for a particular end user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemApplicationListResponse {
    ///A list of connected applications.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<ConnectedApplication>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for ItemApplicationListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}