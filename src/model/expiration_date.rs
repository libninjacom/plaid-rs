
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum ExpirationDate {
    #[serde(rename = "not_expired")]
    NotExpired,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "no_data")]
    NoData,
}