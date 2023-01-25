
use serde::{Serialize, Deserialize};
use super::{MatchSummary, EntityScreeningHitEmails};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitEmailsItems {
    pub analysis: Option<MatchSummary>,
    pub data: Option<EntityScreeningHitEmails>,
}
impl std::fmt::Display for EntityScreeningHitEmailsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}