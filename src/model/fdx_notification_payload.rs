
use serde::{Serialize, Deserialize};
use super::FdxFiAttribute;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxNotificationPayload {
    #[serde(rename = "customFields")]
    pub custom_fields: Option<FdxFiAttribute>,
    pub id: Option<String>,
    #[serde(rename = "idType")]
    pub id_type: Option<String>,
}
impl std::fmt::Display for FdxNotificationPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}