
use serde::{Serialize, Deserialize};
use super::{
    TransferAuthorizationDecisionRationale, TransferAuthorizationGuaranteeDecision,
    TransferAuthorizationGuaranteeDecisionRationale,
    TransferAuthorizationProposedTransfer,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorization {
    pub created: String,
    pub decision: String,
    pub decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    pub id: String,
    pub proposed_transfer: TransferAuthorizationProposedTransfer,
}
impl std::fmt::Display for TransferAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}