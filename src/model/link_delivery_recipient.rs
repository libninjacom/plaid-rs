
use serde::{Serialize, Deserialize};
use super::LinkDeliveryCommunicationMethod;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryRecipient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_methods: Option<Vec<LinkDeliveryCommunicationMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}
impl std::fmt::Display for LinkDeliveryRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}