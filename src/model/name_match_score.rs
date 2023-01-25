
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NameMatchScore {
    pub is_business_name_detected: Option<bool>,
    pub is_first_name_or_last_name_match: Option<bool>,
    pub is_nickname_match: Option<bool>,
    pub score: Option<i64>,
}
impl std::fmt::Display for NameMatchScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}