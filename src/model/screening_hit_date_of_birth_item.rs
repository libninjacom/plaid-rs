
use serde::{Serialize, Deserialize};
use super::{DateRange, MatchSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitDateOfBirthItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DateRange>,
}
impl std::fmt::Display for ScreeningHitDateOfBirthItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}