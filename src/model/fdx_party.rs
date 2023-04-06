
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxParty {
    #[serde(rename = "homeUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_uri: Option<String>,
    #[serde(rename = "logoUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    pub name: String,
    #[serde(rename = "registeredEntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_entity_id: Option<String>,
    #[serde(rename = "registeredEntityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_entity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for FdxParty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}