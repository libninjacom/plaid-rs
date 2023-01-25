
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerEnableResponse {
    pub production_secret: Option<String>,
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerEnableResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}