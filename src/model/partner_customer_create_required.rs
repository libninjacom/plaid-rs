
use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerAddress;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerCreateRequired {
    pub address: PartnerEndCustomerAddress,
    pub application_name: String,
    pub company_name: String,
    pub is_bank_addendum_completed: bool,
    pub is_diligence_attested: bool,
    pub legal_entity_name: String,
    pub products: Vec<String>,
    pub website: String,
}
impl std::fmt::Display for PartnerCustomerCreateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}