
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerBillingContact {
    pub email: Option<String>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerBillingContact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}