
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitsPhoneNumberItems {
    pub analysis: Option<MatchSummary>,
    pub data: Option<EntityScreeningHitPhoneNumbers>,
}
impl std::fmt::Display for EntityScreeningHitsPhoneNumberItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}