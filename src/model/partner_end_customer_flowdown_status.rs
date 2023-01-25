
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PartnerEndCustomerFlowdownStatus {
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "IN_REVIEW")]
    InReview,
    #[serde(rename = "NEGOTIATION")]
    Negotiation,
    #[serde(rename = "COMPLETE")]
    Complete,
}