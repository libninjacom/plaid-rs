use serde::{Serialize, Deserialize};
use super::PartnerEndCustomer;
///Response schema for `/partner/customer/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerGetResponse {
    ///The details for an end customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_customer: Option<PartnerEndCustomer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}