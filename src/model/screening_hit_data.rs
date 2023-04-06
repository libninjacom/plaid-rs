
use serde::{Serialize, Deserialize};
use super::{
    GenericScreeningHitLocationItems, ScreeningHitDateOfBirthItem,
    ScreeningHitDocumentsItems, ScreeningHitNamesItems,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates_of_birth: Option<Vec<ScreeningHitDateOfBirthItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<ScreeningHitDocumentsItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<GenericScreeningHitLocationItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<ScreeningHitNamesItems>>,
}
impl std::fmt::Display for ScreeningHitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}