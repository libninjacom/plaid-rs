use serde::{Serialize, Deserialize};
use super::ValidationSource;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationSources {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "VALIDATION_SOURCE")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_source: Vec<ValidationSource>,
}
impl std::fmt::Display for ValidationSources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}