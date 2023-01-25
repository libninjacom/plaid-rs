
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductAccess {
    pub accounts_details_transactions: Option<bool>,
    pub accounts_routing_number: Option<bool>,
    pub accounts_statements: Option<bool>,
    pub accounts_tax_statements: Option<bool>,
    pub auth: Option<bool>,
    pub customers_profiles: Option<bool>,
    pub identity: Option<bool>,
    pub statements: Option<bool>,
    pub transactions: Option<bool>,
}
impl std::fmt::Display for ProductAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}