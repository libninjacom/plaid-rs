
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitAnalysis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    pub search_terms_version: i64,
}
impl std::fmt::Display for ScreeningHitAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}