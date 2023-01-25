
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerCreateResponse {
    pub end_customer: Option<PartnerEndCustomerWithSecrets>,
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}