
use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerOAuthInstitutionEnvironments;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerOAuthInstitution {
    pub classic_disablement_date: Option<String>,
    pub environments: Option<PartnerEndCustomerOAuthInstitutionEnvironments>,
    pub institution_id: Option<String>,
    pub name: Option<String>,
    pub production_enablement_date: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerOAuthInstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}