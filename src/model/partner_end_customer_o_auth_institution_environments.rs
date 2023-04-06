
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerOAuthInstitutionEnvironments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub development: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerOAuthInstitutionEnvironments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}