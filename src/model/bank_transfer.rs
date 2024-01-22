use serde::{Serialize, Deserialize};
use super::{
    BankTransferDirection, BankTransferFailure, BankTransferMetadata, BankTransferUser,
};
///Represents a bank transfer within the Bank Transfers API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransfer {
    ///The account ID that should be credited/debited for this bank transfer.
    pub account_id: String,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, e.g. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    ///When `true`, you can still cancel this bank transfer.
    pub cancellable: bool,
    ///The datetime when this bank transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: chrono::DateTime<chrono::Utc>,
    ///A string containing the custom tag provided by the client in the create request. Will be null if not provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_tag: Option<String>,
    ///The description of the transfer.
    pub description: String,
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<BankTransferDirection>,
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<BankTransferFailure>,
    ///Plaid’s unique identifier for a bank transfer.
    pub id: String,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
The JSON values must be Strings (no nested JSON objects allowed)
Only ASCII characters may be used
Maximum of 50 key/value pairs
Maximum key length of 40 characters
Maximum value length of 500 characters*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BankTransferMetadata>,
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: String,
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///The status of the transfer.
    pub status: String,
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    #[serde(rename = "type")]
    pub type_: String,
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
}
impl std::fmt::Display for BankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}