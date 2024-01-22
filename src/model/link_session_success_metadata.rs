use serde::{Serialize, Deserialize};
use super::{LinkSessionSuccessMetadataAccount, LinkSessionSuccessMetadataInstitution};
///Displayed once a user has successfully linked their Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionSuccessMetadata {
    ///A list of accounts attached to the connected Item. If Account Select is enabled via the developer dashboard, `accounts` will only include selected accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<LinkSessionSuccessMetadataAccount>>,
    ///An institution object. If the Item was created via Same-Day micro-deposit verification, will be `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkSessionSuccessMetadataInstitution>,
    ///A unique identifier associated with a user's actions and events through the Link flow. Include this identifier when opening a support ticket for faster turnaround.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    /**The status of a transfer. Returned only when [Transfer UI](/docs/transfer/using-transfer-ui) is implemented.

- `COMPLETE` – The transfer was completed.
- `INCOMPLETE` – The transfer could not be completed. For help, see [Troubleshooting transfers](/docs/transfer/using-transfer-ui#troubleshooting-transfers).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<String>,
}
impl std::fmt::Display for LinkSessionSuccessMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}