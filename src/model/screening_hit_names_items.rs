use serde::{Serialize, Deserialize};
use super::{IndividualScreeningHitNames, MatchSummary};
///Analyzed name information for the associated hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitNamesItems {
    ///Summary object reflecting the match result of the associated data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    ///Name information for the associated individual watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<IndividualScreeningHitNames>,
}
impl std::fmt::Display for ScreeningHitNamesItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}