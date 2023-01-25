
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Taxform {
    pub doc_id: Option<String>,
    pub document_type: String,
    pub w2: Option<W2>,
}
impl std::fmt::Display for Taxform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}