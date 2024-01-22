use serde::{Serialize, Deserialize};
use super::{EntityDocument, MatchSummary};
///Analyzed documents for the associated hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitDocumentsItems {
    ///Summary object reflecting the match result of the associated data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    ///An official document, usually issued by a governing body or institution, with an associated identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EntityDocument>,
}
impl std::fmt::Display for EntityScreeningHitDocumentsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}