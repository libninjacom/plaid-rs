use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceProductFulfillmentDetail {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ServiceProductFulfillmentIdentifier")]
    pub service_product_fulfillment_identifier: String,
    ///A string that uniquely identifies a type of order Verification of Asset.
    #[serde(rename = "VendorOrderIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_order_identifier: Option<String>,
}
impl std::fmt::Display for ServiceProductFulfillmentDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}