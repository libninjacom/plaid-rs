use serde::{Serialize, Deserialize};
use super::Scopes;
///Describes a consent activity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Activity {
    ///Types of consent activities
    pub activity: String,
    ///A unique identifier for the activity
    pub id: String,
    ///The date this activity was initiated [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub initiated_date: chrono::NaiveDate,
    ///Application ID of the client who initiated the activity.
    pub initiator: String,
    ///The scopes object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Scopes>,
    ///Enum representing the state of the action/activity.
    pub state: String,
    ///This field will map to the application ID that is returned from /item/application/list, or provided to the institution in an oauth redirect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_application_id: Option<String>,
}
impl std::fmt::Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}