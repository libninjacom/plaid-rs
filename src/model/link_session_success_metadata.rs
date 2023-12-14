
use serde::{Serialize, Deserialize};
use super::{LinkSessionSuccessMetadataAccount, LinkSessionSuccessMetadataInstitution};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionSuccessMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<LinkSessionSuccessMetadataAccount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkSessionSuccessMetadataInstitution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<String>,
}
impl std::fmt::Display for LinkSessionSuccessMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}