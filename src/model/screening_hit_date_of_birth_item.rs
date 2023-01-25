
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitDateOfBirthItem {
    pub analysis: Option<MatchSummary>,
    pub data: Option<DateRange>,
}
impl std::fmt::Display for ScreeningHitDateOfBirthItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}