
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_auth_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_optional_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_payment_initiation_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_numbers: Option<Vec<String>>,
}
impl std::fmt::Display for InstitutionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}