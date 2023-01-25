
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningHitData {
    pub dates_of_birth: Option<Vec<ScreeningHitDateOfBirthItem>>,
    pub documents: Option<Vec<ScreeningHitDocumentsItems>>,
    pub locations: Option<Vec<GenericScreeningHitLocationItems>>,
    pub names: Option<Vec<ScreeningHitNamesItems>>,
}
impl std::fmt::Display for ScreeningHitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}