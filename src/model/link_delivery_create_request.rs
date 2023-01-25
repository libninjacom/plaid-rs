
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryCreateRequest {
    pub delivery_destination: String,
    pub delivery_method: String,
    pub link_token: String,
}
impl std::fmt::Display for LinkDeliveryCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}