
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationSource {
    #[serde(rename = "ValidationSourceName")]
    pub validation_source_name: Option<String>,
    #[serde(rename = "ValidationSourceReferenceIdentifier")]
    pub validation_source_reference_identifier: Option<String>,
}
impl std::fmt::Display for ValidationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}