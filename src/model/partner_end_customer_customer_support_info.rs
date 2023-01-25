
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerCustomerSupportInfo {
    pub contact_url: Option<String>,
    pub email: Option<String>,
    pub link_update_url: Option<String>,
    pub phone_number: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerCustomerSupportInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}