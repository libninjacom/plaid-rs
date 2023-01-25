
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityVerificationStepStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "waiting_for_prerequisite")]
    WaitingForPrerequisite,
    #[serde(rename = "not_applicable")]
    NotApplicable,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "manually_approved")]
    ManuallyApproved,
    #[serde(rename = "manually_rejected")]
    ManuallyRejected,
}