
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxParty {
    #[serde(rename = "homeUri")]
    pub home_uri: Option<String>,
    #[serde(rename = "logoUri")]
    pub logo_uri: Option<String>,
    pub name: String,
    #[serde(rename = "registeredEntityId")]
    pub registered_entity_id: Option<String>,
    #[serde(rename = "registeredEntityName")]
    pub registered_entity_name: Option<String>,
    pub registry: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for FdxParty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}