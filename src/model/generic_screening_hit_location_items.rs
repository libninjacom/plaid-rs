
use serde::{Serialize, Deserialize};
use super::{MatchSummary, WatchlistScreeningHitLocations};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenericScreeningHitLocationItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WatchlistScreeningHitLocations>,
}
impl std::fmt::Display for GenericScreeningHitLocationItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}