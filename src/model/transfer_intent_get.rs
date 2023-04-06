
use serde::{Serialize, Deserialize};
use super::{
    TransferAuthorizationDecisionRationale, TransferAuthorizationGuaranteeDecision,
    TransferAuthorizationGuaranteeDecisionRationale, TransferIntentGetFailureReason,
    TransferMetadata, TransferUserInResponse,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentGet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<String>,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_decision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TransferIntentGetFailureReason>,
    pub funding_account_id: String,
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
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub origination_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_guarantee: Option<bool>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for TransferIntentGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}