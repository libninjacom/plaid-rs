
use serde::{Serialize, Deserialize};
use super::PartnerEndCustomer;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_customer: Option<PartnerEndCustomer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}