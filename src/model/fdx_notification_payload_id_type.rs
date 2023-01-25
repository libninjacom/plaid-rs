
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum FdxNotificationPayloadIdType {
    #[serde(rename = "ACCOUNT")]
    Account,
    #[serde(rename = "CUSTOMER")]
    Customer,
    #[serde(rename = "PARTY")]
    Party,
    #[serde(rename = "MAINTENANCE")]
    Maintenance,
    #[serde(rename = "CONSENT")]
    Consent,
}