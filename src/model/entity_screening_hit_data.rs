
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitData {
    pub documents: Option<Vec<EntityScreeningHitDocumentsItems>>,
    pub email_addresses: Option<Vec<EntityScreeningHitEmailsItems>>,
    pub locations: Option<Vec<GenericScreeningHitLocationItems>>,
    pub names: Option<Vec<EntityScreeningHitNamesItems>>,
    pub phone_numbers: Option<Vec<EntityScreeningHitsPhoneNumberItems>>,
    pub urls: Option<Vec<EntityScreeningHitUrlsItems>>,
}
impl std::fmt::Display for EntityScreeningHitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}