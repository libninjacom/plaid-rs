
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDocumentExtractedDataAnalysis {
    pub date_of_birth: String,
    pub expiration_date: String,
    pub issuing_country: String,
    pub name: String,
}
impl std::fmt::Display for PhysicalDocumentExtractedDataAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}