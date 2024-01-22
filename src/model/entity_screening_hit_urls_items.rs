use serde::{Serialize, Deserialize};
use super::{EntityScreeningHitUrls, MatchSummary};
///Analyzed URLs for the associated hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitUrlsItems {
    ///Summary object reflecting the match result of the associated data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    ///URLs associated with the entity screening hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EntityScreeningHitUrls>,
}
impl std::fmt::Display for EntityScreeningHitUrlsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}