
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum FdxNotificationSeverity {
    #[serde(rename = "EMERGENCY")]
    Emergency,
    #[serde(rename = "ALERT")]
    Alert,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "NOTICE")]
    Notice,
    #[serde(rename = "INFO")]
    Info,
}