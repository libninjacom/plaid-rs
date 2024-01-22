use serde::{Serialize, Deserialize};
use super::{
    TransferAuthorizationDecisionRationale, TransferAuthorizationGuaranteeDecision,
    TransferAuthorizationGuaranteeDecisionRationale, TransferAuthorizationPaymentRisk,
    TransferAuthorizationProposedTransfer,
};
///Contains the authorization decision for a proposed transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorization {
    ///The datetime representing when the authorization was created, in the format `2006-01-02T15:04:05Z`.
    pub created: chrono::DateTime<chrono::Utc>,
    /**A decision regarding the proposed transfer.

`approved` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended (i.e., use Link in update to re-authenticate your user when `decision_rationale.code` is `ITEM_LOGIN_REQUIRED`). Refer to the `code` field in the `decision_rationale` object for details.

`declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.*/
    pub decision: String,
    ///The rationale for Plaid's decision regarding a proposed transfer. It is always set for `declined` decisions, and may or may not be null for `approved` decisions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    ///Indicates whether the transfer is guaranteed by Plaid (Guarantee customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    ///Plaid’s unique identifier for a transfer authorization.
    pub id: String,
    ///This object includes the scores and risk level. This response is offered as an add-on to /transfer/authorization/create. To request access to these fields please contact your Plaid account manager.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_risk: Option<TransferAuthorizationPaymentRisk>,
    ///Details regarding the proposed transfer.
    pub proposed_transfer: TransferAuthorizationProposedTransfer,
}
impl std::fmt::Display for TransferAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}