
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenericScreeningHitLocationItems {
    pub analysis: Option<MatchSummary>,
    pub data: Option<WatchlistScreeningHitLocations>,
}
impl std::fmt::Display for GenericScreeningHitLocationItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}