
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoanIdentifier {
    #[serde(rename = "LoanIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_identifier: Option<String>,
    #[serde(rename = "LoanIdentifierType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_identifier_type: Option<String>,
}
impl std::fmt::Display for LoanIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}