
use serde::{Serialize, Deserialize};
use super::FdxFiAttribute;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxNotificationPayload {
    #[serde(rename = "customFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<FdxFiAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "idType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
}
impl std::fmt::Display for FdxNotificationPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}