use serde::{Serialize, Deserialize};
use super::{EntityScreeningHitEmails, MatchSummary};
///Analyzed emails for the associated hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitEmailsItems {
    ///Summary object reflecting the match result of the associated data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<MatchSummary>,
    ///Email address information for the associated entity watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EntityScreeningHitEmails>,
}
impl std::fmt::Display for EntityScreeningHitEmailsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}