
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDocumentExtractedData {
    pub category: String,
    pub expiration_date: Option<String>,
    pub id_number: Option<String>,
    pub issuing_country: String,
    pub issuing_region: Option<String>,
}
impl std::fmt::Display for PhysicalDocumentExtractedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}