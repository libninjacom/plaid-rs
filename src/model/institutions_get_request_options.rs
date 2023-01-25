
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetRequestOptions {
    pub include_auth_metadata: Option<bool>,
    pub include_optional_metadata: Option<bool>,
    pub include_payment_initiation_metadata: Option<bool>,
    pub oauth: Option<bool>,
    pub products: Option<Vec<String>>,
    pub routing_numbers: Option<Vec<String>>,
}
impl std::fmt::Display for InstitutionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}