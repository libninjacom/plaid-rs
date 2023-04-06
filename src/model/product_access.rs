
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_details_transactions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_routing_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_statements: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_tax_statements: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customers_profiles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<bool>,
}
impl std::fmt::Display for ProductAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}