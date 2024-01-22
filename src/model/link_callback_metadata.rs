use serde::{Serialize, Deserialize};
use super::{LinkDeliveryAccount, LinkDeliveryInstitution};
///Information related to the callback from the Hosted Link session.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkCallbackMetadata {
    ///A list of accounts attached to the connected Item. If Account Select is enabled via the developer dashboard, accounts will only include selected accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<LinkDeliveryAccount>>,
    ///The type of Link callback event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_type: Option<String>,
    ///A string representing the event that has just occurred in the Link flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    ///Information related to the financial institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkDeliveryInstitution>,
    ///A unique identifier associated with a user's actions and events through the Link flow. Include this identifier when opening a support ticket for faster turnaround.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    ///The request ID for the last request made by Link. This can be shared with Plaid Support to expedite investigation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    ///Indicates where in the flow the Link user exited
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for LinkCallbackMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}