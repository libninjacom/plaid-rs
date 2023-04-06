
use serde::{Serialize, Deserialize};
use super::{EntityScreeningHitPhoneNumbers, MatchSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitsPhoneNumberItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EntityScreeningHitPhoneNumbers>,
}
impl std::fmt::Display for EntityScreeningHitsPhoneNumberItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}