
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitDocumentsItems {
    pub analysis: Option<MatchSummary>,
    pub data: Option<WatchlistScreeningDocument>,
}
impl std::fmt::Display for ScreeningHitDocumentsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}