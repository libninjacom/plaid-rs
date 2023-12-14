
use serde::{Serialize, Deserialize};
use super::{
    TransferAuthorizationGuaranteeDecision,
    TransferAuthorizationGuaranteeDecisionRationale, TransferCreditFundsSource,
    TransferExpectedSweepSettlementScheduleItem, TransferFailure, TransferMetadata,
    TransferRefund, TransferSweepStatus, TransferUserInResponse,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<String>,
    pub amount: String,
    pub authorization_id: String,
    pub cancellable: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    pub credit_funds_source: TransferCreditFundsSource,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_settlement_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_sweep_settlement_schedule: Option<
        Vec<TransferExpectedSweepSettlementScheduleItem>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facilitator_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TransferFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    pub id: String,
    pub iso_currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TransferMetadata>,
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_trace_id: Option<String>,
    pub origination_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_transfer_id: Option<String>,
    pub refunds: Vec<TransferRefund>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_return_window: Option<chrono::NaiveDate>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweep_status: Option<TransferSweepStatus>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthorized_return_window: Option<chrono::NaiveDate>,
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}