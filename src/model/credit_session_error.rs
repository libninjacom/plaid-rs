use serde::{Serialize, Deserialize};
///The details of a Link error.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionError {
    ///A user-friendly representation of the error code. `null` if the error is not related to user action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_message: Option<String>,
    ///The particular error code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    ///A developer-friendly representation of the error code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    ///A broad categorization of the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}
impl std::fmt::Display for CreditSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}