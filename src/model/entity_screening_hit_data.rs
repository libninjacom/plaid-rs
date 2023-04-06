
use serde::{Serialize, Deserialize};
use super::{
    EntityScreeningHitDocumentsItems, EntityScreeningHitEmailsItems,
    EntityScreeningHitNamesItems, EntityScreeningHitUrlsItems,
    EntityScreeningHitsPhoneNumberItems, GenericScreeningHitLocationItems,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<EntityScreeningHitDocumentsItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<EntityScreeningHitEmailsItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<GenericScreeningHitLocationItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<EntityScreeningHitNamesItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<EntityScreeningHitsPhoneNumberItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<EntityScreeningHitUrlsItems>>,
}
impl std::fmt::Display for EntityScreeningHitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}