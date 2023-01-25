
use serde::{Serialize, Deserialize};
use super::Activity;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemActivityListResponse {
    pub activities: Vec<Activity>,
    pub cursor: Option<String>,
    pub request_id: Option<String>,
}
impl std::fmt::Display for ItemActivityListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}