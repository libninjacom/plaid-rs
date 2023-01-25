
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum SignalDecisionOutcome {
    #[serde(rename = "APPROVE")]
    Approve,
    #[serde(rename = "REVIEW")]
    Review,
    #[serde(rename = "REJECT")]
    Reject,
    #[serde(rename = "TAKE_OTHER_RISK_MEASURES")]
    TakeOtherRiskMeasures,
    #[serde(rename = "NOT_EVALUATED")]
    NotEvaluated,
}