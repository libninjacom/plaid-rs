
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningDocumentType {
    #[serde(rename = "birth_certificate")]
    BirthCertificate,
    #[serde(rename = "drivers_license")]
    DriversLicense,
    #[serde(rename = "immigration_number")]
    ImmigrationNumber,
    #[serde(rename = "military_id")]
    MilitaryId,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "personal_identification")]
    PersonalIdentification,
    #[serde(rename = "ration_card")]
    RationCard,
    #[serde(rename = "ssn")]
    Ssn,
    #[serde(rename = "student_id")]
    StudentId,
    #[serde(rename = "tax_id")]
    TaxId,
    #[serde(rename = "travel_document")]
    TravelDocument,
    #[serde(rename = "voter_id")]
    VoterId,
}