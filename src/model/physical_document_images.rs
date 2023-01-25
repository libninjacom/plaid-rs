
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDocumentImages {
    pub cropped_back: Option<String>,
    pub cropped_front: Option<String>,
    pub face: Option<String>,
    pub original_back: Option<String>,
    pub original_front: String,
}
impl std::fmt::Display for PhysicalDocumentImages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}