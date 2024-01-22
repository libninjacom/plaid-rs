use serde::{Serialize, Deserialize};
///Information related to the related to the delivery of the link session to users
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryMetadata {
    ///The communication method used to deliver the Hosted Link session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communication_method: Option<String>,
    ///The status of the delivery of the Hosted Link to the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
}
impl std::fmt::Display for LinkDeliveryMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}