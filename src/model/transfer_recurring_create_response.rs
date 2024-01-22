use serde::{Serialize, Deserialize};
use super::{RecurringTransferNullable, TransferAuthorizationDecisionRationale};
///Defines the response schema for `/transfer/recurring/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringCreateResponse {
    /**A decision regarding the proposed transfer.

`approved` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended (i.e., use Link in update to re-authenticate your user when `decision_rationale.code` is `ITEM_LOGIN_REQUIRED`). Refer to the `code` field in the `decision_rationale` object for details.

`declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.*/
    pub decision: String,
    ///The rationale for Plaid's decision regarding a proposed transfer. It is always set for `declined` decisions, and may or may not be null for `approved` decisions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    ///Represents a recurring transfer within the Transfers API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_transfer: Option<RecurringTransferNullable>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferRecurringCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}