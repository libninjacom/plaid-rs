use serde::{Serialize, Deserialize};
///The details for the newly created end customer, including secrets for non-Production environments.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerWithSecrets {}
impl std::fmt::Display for PartnerEndCustomerWithSecrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}