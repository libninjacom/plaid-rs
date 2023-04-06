
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDocumentExtractedData {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    pub issuing_country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_region: Option<String>,
}
impl std::fmt::Display for PhysicalDocumentExtractedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}