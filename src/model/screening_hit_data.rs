use serde::{Serialize, Deserialize};
use super::{
    GenericScreeningHitLocationItems, ScreeningHitDateOfBirthItem,
    ScreeningHitDocumentsItems, ScreeningHitNamesItems,
};
///Information associated with the watchlist hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitData {
    ///Dates of birth associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates_of_birth: Option<Vec<ScreeningHitDateOfBirthItem>>,
    ///Documents associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<ScreeningHitDocumentsItems>>,
    ///Locations associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<GenericScreeningHitLocationItems>>,
    ///Names associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<ScreeningHitNamesItems>>,
}
impl std::fmt::Display for ScreeningHitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}