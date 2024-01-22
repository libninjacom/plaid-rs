use serde::{Serialize, Deserialize};
use super::LinkDeliveryRecipient;
///Optional metadata related to the Hosted Link session
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryOptions {
    ///Metadata related to the recipient. If the information required to populate this field is not available, leave it blank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<LinkDeliveryRecipient>,
}
impl std::fmt::Display for LinkDeliveryOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}