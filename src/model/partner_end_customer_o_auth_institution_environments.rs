
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerOAuthInstitutionEnvironments {
    pub development: Option<String>,
    pub production: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerOAuthInstitutionEnvironments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}