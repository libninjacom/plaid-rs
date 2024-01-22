use serde::{Serialize, Deserialize};
use super::{Activity, LastDataAccessTimes};
///Describes a historical log of user consent events.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemActivityListResponse {
    ///A list of activities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activities: Vec<Activity>,
    ///Cursor used for pagination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    ///An array of objects containing timestamps for the last time each data type was accessed per application.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub last_data_access_times: Vec<LastDataAccessTimes>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemActivityListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}