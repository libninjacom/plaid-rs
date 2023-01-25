
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PartnerEndCustomerQuestionnaireStatus {
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "COMPLETE")]
    Complete,
}