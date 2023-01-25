
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationConsentStatus {
    #[serde(rename = "UNAUTHORISED")]
    Unauthorised,
    #[serde(rename = "AUTHORISED")]
    Authorised,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "EXPIRED")]
    Expired,
}