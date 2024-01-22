use serde::{Serialize, Deserialize};
use super::TransferFailure;
///Represents an event in the Transfers API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferEvent {
    ///The account ID associated with the transfer. This field is omitted for Plaid Ledger Sweep events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    pub event_id: i64,
    /**The type of event that this transfer represents. Event types with prefix `sweep` represents events for Plaid Ledger sweeps.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`settled`: Credits are available to be withdrawn or debits have been deducted from the Plaid linked account.

`returned`: A posted transfer was returned.

`swept`: The transfer was swept to / from the sweep account.

`swept_settled`: Credits are available to be withdrawn or debits have been deducted from the customer’s business checking account.

`return_swept`: Due to the transfer being returned, funds were pulled from or pushed back to the sweep account.

`sweep.pending`: A new ledger sweep was created; it is in the pending state.

`sweep.posted`: The ledger sweep has been successfully submitted to the payment network.

`sweep.settled`: The transaction has settled in the funding account. This means that funds withdrawn from Plaid Ledger balance have reached the funding account, or funds to be deposited into the Plaid Ledger Balance have been pulled, and the hold period has begun.

`sweep.returned`: A posted ledger sweep was returned.

`sweep.failed`: The ledger sweep failed, no funds were moved.

`refund.pending`: A new refund was created; it is in the pending state.

`refund.cancelled`: The refund was cancelled.

`refund.failed`: The refund failed, no funds were moved.

`refund.posted`: The refund has been successfully submitted to the payment network.

`refund.settled`: The refund transaction has settled in the Plaid linked account.

`refund.returned`: A posted refund was returned.

`refund.swept`: The refund was swept from the sweep account.

`refund.return_swept`: Due to the refund being returned, funds were pushed back to the sweep account.*/
    pub event_type: String,
    ///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TransferFailure>,
    ///The id of the associated funding account, available in the Plaid Dashboard. If present, this indicates which of your business checking accounts will be credited or debited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<String>,
    ///The ID of the origination account that this balance belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<String>,
    ///The Plaid client ID that is the originator of the transfer that this event applies to. Only present if the transfer was created on behalf of another client as a third-party sender (TPS).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<String>,
    ///Plaid’s unique identifier for a refund. A non-null value indicates the event is for the associated refund of the transfer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    ///A signed amount of how much was `swept` or `return_swept` for this transfer (decimal string with two digits of precision e.g. "-5.50").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sweep_amount: Option<String>,
    ///Plaid’s unique identifier for a sweep.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sweep_id: Option<String>,
    ///The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    pub timestamp: chrono::DateTime<chrono::Utc>,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00"). This field is omitted for Plaid Ledger Sweep events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<String>,
    ///Plaid’s unique identifier for a transfer. This field is `null` for Plaid Ledger Sweep events.
    pub transfer_id: String,
    ///The type of transfer. Valid values are `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account. This field is omitted for Plaid Ledger Sweep events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}
impl std::fmt::Display for TransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}