
use serde::{Serialize, Deserialize};
use super::LinkSessionExitMetadataInstitution;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionExitMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkSessionExitMetadataInstitution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for LinkSessionExitMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}