
use serde::{Serialize, Deserialize};
use super::{Activity, LastDataAccessTimes};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemActivityListResponse {
    pub activities: Vec<Activity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub last_data_access_times: Vec<LastDataAccessTimes>,
    pub request_id: String,
}
impl std::fmt::Display for ItemActivityListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}