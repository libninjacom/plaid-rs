use serde::{Serialize, Deserialize};
use super::Scopes;
///Describes the connected application for a particular end user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectedApplication {
    ///This field will map to the application ID that is returned from /item/application/list, or provided to the institution in an oauth redirect.
    pub application_id: String,
    ///The URL for the application's website
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_url: Option<String>,
    ///The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///A human-readable name of the application for display purposes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    ///A URL that links to the application logo image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    ///The name of the application
    pub name: String,
    ///A string provided by the connected app stating why they use their respective enabled products.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_for_access: Option<String>,
    ///The scopes object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Scopes>,
}
impl std::fmt::Display for ConnectedApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}