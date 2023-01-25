
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductStatusBreakdown {
    pub error_institution: f64,
    pub error_plaid: f64,
    pub refresh_interval: Option<String>,
    pub success: f64,
}
impl std::fmt::Display for ProductStatusBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}