
use serde::{Serialize, Deserialize};
use super::{
    TransferAuthorizationGuaranteeDecisionRationale, TransferIntentGetFailureReason,
    TransferAuthorizationGuaranteeDecision, TransferMetadata, TransferUserInResponse,
    TransferAuthorizationDecisionRationale,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentGet {
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: String,
    pub authorization_decision: Option<String>,
    pub authorization_decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    pub created: String,
    pub description: String,
    pub failure_reason: Option<TransferIntentGetFailureReason>,
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    pub id: String,
    pub iso_currency_code: String,
    pub metadata: Option<TransferMetadata>,
    pub mode: String,
    pub origination_account_id: String,
    pub require_guarantee: Option<bool>,
    pub status: String,
    pub transfer_id: Option<String>,
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for TransferIntentGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}