
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceProductFulfillment {
    #[serde(rename = "SERVICE_PRODUCT_FULFILLMENT_DETAIL")]
    pub service_product_fulfillment_detail: ServiceProductFulfillmentDetail,
}
impl std::fmt::Display for ServiceProductFulfillment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}