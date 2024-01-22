use serde::{Serialize, Deserialize};
use super::{EntityScreeningHitPhoneNumbers, MatchSummary};
///Analyzed phone numbers for the associated hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitsPhoneNumberItems {
    ///Summary object reflecting the match result of the associated data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    ///Phone number information associated with the entity screening hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EntityScreeningHitPhoneNumbers>,
}
impl std::fmt::Display for EntityScreeningHitsPhoneNumberItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}