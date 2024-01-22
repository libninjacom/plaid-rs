use serde::{Serialize, Deserialize};
///The communication method containing both the type and address to send the URL.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryCommunicationMethod {
    ///The phone number / email address that Hosted Link sessions are delivered to. Phone numbers must be in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /**The delivery method to be used to deliver the Hosted Link session URL.

`SMS`: The URL will be delivered through SMS

`EMAIL`: The URL will be delivered through email*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
impl std::fmt::Display for LinkDeliveryCommunicationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}