
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityUpdateTypes {
    #[serde(rename = "PHONES")]
    Phones,
    #[serde(rename = "ADDRESSES")]
    Addresses,
    #[serde(rename = "EMAILS")]
    Emails,
    #[serde(rename = "NAMES")]
    Names,
}