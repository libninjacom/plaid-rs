
use serde::{Serialize, Deserialize};
use super::ValidationSource;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationSources {
    #[serde(rename = "VALIDATION_SOURCE")]
    pub validation_source: Vec<ValidationSource>,
}
impl std::fmt::Display for ValidationSources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}