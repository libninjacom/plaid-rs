
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiabilitiesAccountIdsWithUpdatedLiabilities {}
impl std::fmt::Display for LiabilitiesAccountIdsWithUpdatedLiabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}