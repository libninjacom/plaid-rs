
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Application {
    pub application_id: String,
    pub application_url: Option<String>,
    pub city: Option<String>,
    pub company_legal_name: Option<String>,
    pub country_code: Option<String>,
    pub display_name: Option<String>,
    pub join_date: String,
    pub logo_url: Option<String>,
    pub name: String,
    pub postal_code: Option<String>,
    pub reason_for_access: Option<String>,
    pub region: Option<String>,
    pub use_case: Option<String>,
}
impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}