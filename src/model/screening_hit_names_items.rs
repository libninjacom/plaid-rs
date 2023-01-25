
use serde::{Serialize, Deserialize};
use super::{IndividualScreeningHitNames, MatchSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitNamesItems {
    pub analysis: Option<MatchSummary>,
    pub data: Option<IndividualScreeningHitNames>,
}
impl std::fmt::Display for ScreeningHitNamesItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}