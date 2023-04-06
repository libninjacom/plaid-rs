
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDocumentImages {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cropped_back: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cropped_front: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_back: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_front: Option<String>,
}
impl std::fmt::Display for PhysicalDocumentImages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}