
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerGetResponse {
    pub end_customer: Option<PartnerEndCustomer>,
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}