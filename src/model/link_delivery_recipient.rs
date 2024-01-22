use serde::{Serialize, Deserialize};
use super::LinkDeliveryCommunicationMethod;
///Metadata related to the recipient. If the information required to populate this field is not available, leave it blank.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryRecipient {
    ///The list of communication methods to send the Hosted Link session URL to. If delivery is not required, leave this field blank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communication_methods: Option<Vec<LinkDeliveryCommunicationMethod>>,
    ///First name of the recipient. Will be used in the body of the email / text (if configured). If this information is not available, leave this field blank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}
impl std::fmt::Display for LinkDeliveryRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}