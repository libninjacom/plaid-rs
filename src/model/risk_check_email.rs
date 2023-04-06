
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckEmail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breach_count: Option<i64>,
    pub domain_is_custom: String,
    pub domain_is_disposable: String,
    pub domain_is_free_provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registered_at: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_breached_at: Option<chrono::NaiveDate>,
    pub is_deliverable: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_breached_at: Option<chrono::NaiveDate>,
    pub linked_services: Vec<String>,
    pub top_level_domain_is_suspicious: String,
}
impl std::fmt::Display for RiskCheckEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}