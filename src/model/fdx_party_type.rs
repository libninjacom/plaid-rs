
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum FdxPartyType {
    #[serde(rename = "DATA_ACCESS_PLATFORM")]
    DataAccessPlatform,
    #[serde(rename = "DATA_PROVIDER")]
    DataProvider,
    #[serde(rename = "DATA_RECIPIENT")]
    DataRecipient,
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    #[serde(rename = "MERCHANT")]
    Merchant,
    #[serde(rename = "VENDOR")]
    Vendor,
}