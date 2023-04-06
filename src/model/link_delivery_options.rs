
use serde::{Serialize, Deserialize};
use super::LinkDeliveryRecipient;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<LinkDeliveryRecipient>,
}
impl std::fmt::Display for LinkDeliveryOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}