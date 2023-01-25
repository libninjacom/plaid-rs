
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PartnerEndCustomerStatus {
    #[serde(rename = "UNDER_REVIEW")]
    UnderReview,
    #[serde(rename = "PENDING_ENABLEMENT")]
    PendingEnablement,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DENIED")]
    Denied,
}