
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerTechnicalContact {
    pub email: Option<String>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerTechnicalContact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}