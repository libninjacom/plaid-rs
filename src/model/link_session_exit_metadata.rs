use serde::{Serialize, Deserialize};
use super::LinkSessionExitMetadataInstitution;
///Displayed if a user exits Link without successfully linking an Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionExitMetadata {
    ///An institution object. If the Item was created via Same-Day micro-deposit verification, will be `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkSessionExitMetadataInstitution>,
    ///A unique identifier associated with a user's actions and events through the Link flow. Include this identifier when opening a support ticket for faster turnaround.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    ///The request ID for the last request made by Link. This can be shared with Plaid Support to expedite investigation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    ///The point at which the user exited the Link flow. One of the following values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for LinkSessionExitMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}