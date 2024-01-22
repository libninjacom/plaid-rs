use serde::{Serialize, Deserialize};
use super::ServiceProductFulfillmentDetail;
///A collection of details related to a fulfillment service or product in terms of request, process and result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceProductFulfillment {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "SERVICE_PRODUCT_FULFILLMENT_DETAIL")]
    pub service_product_fulfillment_detail: ServiceProductFulfillmentDetail,
}
impl std::fmt::Display for ServiceProductFulfillment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}