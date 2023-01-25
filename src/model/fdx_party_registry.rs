
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum FdxPartyRegistry {
    #[serde(rename = "FDX")]
    Fdx,
    #[serde(rename = "GLEIF")]
    Gleif,
    #[serde(rename = "ICANN")]
    Icann,
    #[serde(rename = "PRIVATE")]
    Private,
}