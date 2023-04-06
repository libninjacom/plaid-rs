
use serde::{Serialize, Deserialize};
use super::PlaidError;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    pub available_products: Vec<String>,
    pub billed_products: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consented_products: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    pub update_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}