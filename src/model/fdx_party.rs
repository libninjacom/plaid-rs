use serde::{Serialize, Deserialize};
///FDX Participant - an entity or person that is a part of a FDX API transaction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxParty {
    ///URI for party, where an end user could learn more about the company or application involved in the data sharing chain
    #[serde(rename = "homeUri")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_uri: Option<String>,
    ///URI for a logo asset to be displayed to the end user
    #[serde(rename = "logoUri")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    ///Human recognizable common name
    pub name: String,
    ///Registered id of party
    #[serde(rename = "registeredEntityId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registered_entity_id: Option<String>,
    ///Registered name of party
    #[serde(rename = "registeredEntityName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registered_entity_name: Option<String>,
    ///The registry containing the party’s registration with name and id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    ///Identifies the type of a party
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for FdxParty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}