use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerWithSecrets;
///Response schema for `/partner/customer/create`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerCreateResponse {
    ///The details for the newly created end customer, including secrets for non-Production environments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_customer: Option<PartnerEndCustomerWithSecrets>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}