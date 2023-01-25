
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaidError {
    pub causes: Option<Vec<serde_json::Value>>,
    pub display_message: Option<String>,
    pub documentation_url: Option<String>,
    pub error_code: String,
    pub error_message: String,
    pub error_type: String,
    pub request_id: Option<String>,
    pub status: Option<f64>,
    pub suggested_action: Option<String>,
}
impl std::fmt::Display for PlaidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}