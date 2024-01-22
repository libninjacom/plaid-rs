use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationSource {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ValidationSourceName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_source_name: Option<String>,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ValidationSourceReferenceIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_source_reference_identifier: Option<String>,
}
impl std::fmt::Display for ValidationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}