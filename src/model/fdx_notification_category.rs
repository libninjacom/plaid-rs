
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum FdxNotificationCategory {
    #[serde(rename = "SECURITY")]
    Security,
    #[serde(rename = "MAINTENANCE")]
    Maintenance,
    #[serde(rename = "FRAUD")]
    Fraud,
    #[serde(rename = "CONSENT")]
    Consent,
    #[serde(rename = "NEW_DATA")]
    NewData,
}