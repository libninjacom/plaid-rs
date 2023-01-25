
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PhysicalDocumentCategory {
    #[serde(rename = "drivers_license")]
    DriversLicense,
    #[serde(rename = "id_card")]
    IdCard,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "residence_permit_card")]
    ResidencePermitCard,
    #[serde(rename = "resident_card")]
    ResidentCard,
}