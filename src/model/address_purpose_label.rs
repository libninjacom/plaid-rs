
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum AddressPurposeLabel {
    #[serde(rename = "residential")]
    Residential,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "no_data")]
    NoData,
}