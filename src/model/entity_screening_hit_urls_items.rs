
use serde::{Serialize, Deserialize};
use super::{EntityScreeningHitUrls, MatchSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitUrlsItems {
    pub analysis: Option<MatchSummary>,
    pub data: Option<EntityScreeningHitUrls>,
}
impl std::fmt::Display for EntityScreeningHitUrlsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}