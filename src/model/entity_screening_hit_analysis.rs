
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitAnalysis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<String>,
    pub search_terms_version: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<String>,
}
impl std::fmt::Display for EntityScreeningHitAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}