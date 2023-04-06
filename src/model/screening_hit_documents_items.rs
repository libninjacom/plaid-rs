
use serde::{Serialize, Deserialize};
use super::{MatchSummary, WatchlistScreeningDocument};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitDocumentsItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WatchlistScreeningDocument>,
}
impl std::fmt::Display for ScreeningHitDocumentsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}