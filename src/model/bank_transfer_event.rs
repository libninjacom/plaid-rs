use serde::{Serialize, Deserialize};
use super::{BankTransferDirection, BankTransferFailure};
///Represents an event in the Bank Transfers API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferEvent {
    ///The account ID associated with the bank transfer.
    pub account_id: String,
    ///The bank transfer amount.
    pub bank_transfer_amount: String,
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: String,
    ///The currency of the bank transfer amount.
    pub bank_transfer_iso_currency_code: String,
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub bank_transfer_type: String,
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<BankTransferDirection>,
    ///Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    pub event_id: i64,
    /**The type of event that this bank transfer represents.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`reversed`: A posted transfer was reversed.*/
    pub event_type: String,
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<BankTransferFailure>,
    ///The ID of the origination account that this balance belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<String>,
    ///The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for BankTransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}