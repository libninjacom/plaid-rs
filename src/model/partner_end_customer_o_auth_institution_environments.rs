use serde::{Serialize, Deserialize};
///Registration statuses by environment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerOAuthInstitutionEnvironments {
    ///The registration status for the end customer's application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub development: Option<String>,
    ///The registration status for the end customer's application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerOAuthInstitutionEnvironments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}