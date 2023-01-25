
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    pub available_products: Vec<String>,
    pub billed_products: Vec<String>,
    pub consent_expiration_time: Option<String>,
    pub consented_products: Option<Vec<String>>,
    pub error: Option<PlaidError>,
    pub institution_id: Option<String>,
    pub item_id: String,
    pub products: Option<Vec<String>>,
    pub update_type: String,
    pub webhook: Option<String>,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}