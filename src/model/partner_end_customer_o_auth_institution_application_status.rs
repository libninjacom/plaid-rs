
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PartnerEndCustomerOAuthInstitutionApplicationStatus {
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "ATTENTION_REQUIRED")]
    AttentionRequired,
}