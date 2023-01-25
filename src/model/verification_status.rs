
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationStatus {
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "UNVERIFIED")]
    Unverified,
    #[serde(rename = "NEEDS_INFO")]
    NeedsInfo,
    #[serde(rename = "UNABLE_TO_VERIFY")]
    UnableToVerify,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}