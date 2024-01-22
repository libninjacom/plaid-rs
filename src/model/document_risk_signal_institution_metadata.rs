use serde::{Serialize, Deserialize};
///An object which contains additional metadata about the institution used to compute the verification attribute
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRiskSignalInstitutionMetadata {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
}
impl std::fmt::Display for DocumentRiskSignalInstitutionMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}