
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitAnalysis {
    pub documents: Option<String>,
    pub email_addresses: Option<String>,
    pub locations: Option<String>,
    pub names: Option<String>,
    pub phone_numbers: Option<String>,
    pub search_terms_version: f64,
    pub urls: Option<String>,
}
impl std::fmt::Display for EntityScreeningHitAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}