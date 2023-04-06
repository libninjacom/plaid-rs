
use serde::{Serialize, Deserialize};
use super::{EntityDocument, MatchSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitDocumentsItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EntityDocument>,
}
impl std::fmt::Display for EntityScreeningHitDocumentsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}