
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationSource {
    #[serde(rename = "ValidationSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_source_name: Option<String>,
    #[serde(rename = "ValidationSourceReferenceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_source_reference_identifier: Option<String>,
}
impl std::fmt::Display for ValidationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}