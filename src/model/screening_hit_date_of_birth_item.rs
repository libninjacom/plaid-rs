use serde::{Serialize, Deserialize};
use super::{DateRange, MatchSummary};
///Analyzed date of birth for the associated hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitDateOfBirthItem {
    ///Summary object reflecting the match result of the associated data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    ///A date range with a start and end date
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<DateRange>,
}
impl std::fmt::Display for ScreeningHitDateOfBirthItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}