
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaidError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causes: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    pub error_code: String,
    pub error_message: String,
    pub error_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_action: Option<String>,
}
impl std::fmt::Display for PlaidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}