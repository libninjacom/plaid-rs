
use serde::{Serialize, Deserialize};
use super::{
    TransferMetadata, TransferAuthorizationGuaranteeDecisionRationale,
    TransferUserInResponse, TransferAuthorizationGuaranteeDecision, TransferFailure,
    TransferRefund, TransferSweepStatus,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: String,
    pub cancellable: bool,
    pub created: String,
    pub description: String,
    pub failure_reason: Option<TransferFailure>,
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    pub id: String,
    pub iso_currency_code: String,
    pub metadata: Option<TransferMetadata>,
    pub network: String,
    pub origination_account_id: String,
    pub originator_client_id: Option<String>,
    pub recurring_transfer_id: Option<String>,
    pub refunds: Vec<TransferRefund>,
    pub standard_return_window: Option<String>,
    pub status: String,
    pub sweep_status: Option<TransferSweepStatus>,
    #[serde(rename = "type")]
    pub type_: String,
    pub unauthorized_return_window: Option<String>,
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}