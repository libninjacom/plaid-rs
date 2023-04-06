
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceProductFulfillmentDetail {
    #[serde(rename = "ServiceProductFulfillmentIdentifier")]
    pub service_product_fulfillment_identifier: String,
    #[serde(rename = "VendorOrderIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_order_identifier: Option<String>,
}
impl std::fmt::Display for ServiceProductFulfillmentDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}