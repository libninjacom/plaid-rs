
use serde::{Serialize, Deserialize};
use super::{IndividualScreeningHitNames, MatchSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitNamesItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<IndividualScreeningHitNames>,
}
impl std::fmt::Display for ScreeningHitNamesItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}