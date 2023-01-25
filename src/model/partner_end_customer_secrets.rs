
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerSecrets {
    pub development: Option<String>,
    pub sandbox: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerSecrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}