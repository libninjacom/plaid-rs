
use serde::{Serialize, Deserialize};
use super::{EntityScreeningHitNames, MatchSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitNamesItems {
    pub analysis: Option<MatchSummary>,
    pub data: Option<EntityScreeningHitNames>,
}
impl std::fmt::Display for EntityScreeningHitNamesItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}