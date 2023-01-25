
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentConsentPeriodicAlignment {
    #[serde(rename = "CALENDAR")]
    Calendar,
    #[serde(rename = "CONSENT")]
    Consent,
}