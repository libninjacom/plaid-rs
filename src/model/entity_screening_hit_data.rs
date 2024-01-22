use serde::{Serialize, Deserialize};
use super::{
    EntityScreeningHitDocumentsItems, EntityScreeningHitEmailsItems,
    EntityScreeningHitNamesItems, EntityScreeningHitUrlsItems,
    EntityScreeningHitsPhoneNumberItems, GenericScreeningHitLocationItems,
};
///Information associated with the entity watchlist hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitData {
    ///Documents associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<EntityScreeningHitDocumentsItems>>,
    ///Email addresses associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<EntityScreeningHitEmailsItems>>,
    ///Locations associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<GenericScreeningHitLocationItems>>,
    ///Names associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<EntityScreeningHitNamesItems>>,
    ///Phone numbers associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<EntityScreeningHitsPhoneNumberItems>>,
    ///URLs associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<EntityScreeningHitUrlsItems>>,
}
impl std::fmt::Display for EntityScreeningHitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}