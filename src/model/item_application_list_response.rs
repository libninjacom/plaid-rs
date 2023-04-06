
use serde::{Serialize, Deserialize};
use super::ConnectedApplication;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemApplicationListResponse {
    pub applications: Vec<ConnectedApplication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for ItemApplicationListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}