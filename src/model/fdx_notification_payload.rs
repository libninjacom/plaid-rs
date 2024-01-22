use serde::{Serialize, Deserialize};
use super::FdxFiAttribute;
///Custom key-value pairs payload for a notification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxNotificationPayload {
    #[serde(rename = "customFields")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<FdxFiAttribute>>,
    ///ID for the origination entity related to the notification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Type of entity causing origination of a notification
    #[serde(rename = "idType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
}
impl std::fmt::Display for FdxNotificationPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}