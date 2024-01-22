use serde::{Serialize, Deserialize};
use super::W2;
///Data about an official document used to report the user's income to the IRS.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Taxform {
    ///An identifier of the document referenced by the document metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<String>,
    ///The type of tax document. Currently, the only supported value is `w2`.
    pub document_type: String,
    ///W2 is an object that represents income data taken from a W2 tax document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w2: Option<W2>,
}
impl std::fmt::Display for Taxform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}