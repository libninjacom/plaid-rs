
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityDocumentType {
    #[serde(rename = "bik")]
    Bik,
    #[serde(rename = "business_number")]
    BusinessNumber,
    #[serde(rename = "imo")]
    Imo,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "tax_id")]
    TaxId,
}