
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationRefreshStatus {
    #[serde(rename = "VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED")]
    VerificationRefreshStatusUserPresenceRequired,
    #[serde(rename = "VERIFICATION_REFRESH_SUCCESSFUL")]
    VerificationRefreshSuccessful,
    #[serde(rename = "VERIFICATION_REFRESH_NOT_FOUND")]
    VerificationRefreshNotFound,
}