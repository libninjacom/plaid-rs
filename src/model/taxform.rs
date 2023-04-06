
use serde::{Serialize, Deserialize};
use super::W2;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Taxform {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<String>,
    pub document_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w2: Option<W2>,
}
impl std::fmt::Display for Taxform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}