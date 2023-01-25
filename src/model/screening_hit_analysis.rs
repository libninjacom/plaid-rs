
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitAnalysis {
    pub dates_of_birth: Option<String>,
    pub documents: Option<String>,
    pub locations: Option<String>,
    pub names: Option<String>,
    pub search_terms_version: f64,
}
impl std::fmt::Display for ScreeningHitAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}