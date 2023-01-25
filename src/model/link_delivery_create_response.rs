
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryCreateResponse {
    pub link_delivery_session_id: String,
    pub link_delivery_url: String,
    pub request_id: String,
}
impl std::fmt::Display for LinkDeliveryCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}