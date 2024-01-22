use serde::{Serialize, Deserialize};
use super::{TransferRecurringSchedule, TransferUserInResponse};
///Represents a recurring transfer within the Transfers API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringTransfer {
    ///The Plaid `account_id` corresponding to the end-user account that will be debited or credited.
    pub account_id: String,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, e.g. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<String>,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00"). When calling `/transfer/authorization/create`, specify the maximum amount to authorize. When calling `/transfer/create`, specify the exact amount of the transfer, up to a maximum of the amount authorized. If this field is left blank when calling `/transfer/create`, the maximum amount authorized in the `authorization_id` will be sent.
    pub amount: String,
    ///The datetime when this transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: chrono::DateTime<chrono::Utc>,
    ///The description of the recurring transfer.
    pub description: String,
    ///The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited.
    pub funding_account_id: String,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    /**The network or rails used for the transfer.

For transfers submitted as `ach`, the next-day cutoff is 5:30 PM Eastern Time.

For transfers submitted as `same-day-ach`, the same-day cutoff is 3:30 PM Eastern Time. If the transfer is submitted after this cutoff but before the next-day cutoff, it will be sent over next-day rails and will not incur same-day charges; this will apply to both legs of the transfer if applicable.

For transfers submitted as `rtp`,  Plaid will automatically route between Real Time Payment rail by TCH or FedNow rails as necessary. If a transfer is submitted as `rtp` and the counterparty account is not eligible for RTP, the `/transfer/authorization/create` request will fail with an `INVALID_FIELD` error code. To pre-check to determine whether a counterparty account can support RTP, call `/transfer/capabilities/get` before calling `/transfer/authorization/create`.*/
    pub network: String,
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).

The next transfer origination date after bank holiday adjustment.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_origination_date: Option<chrono::NaiveDate>,
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///Plaid’s unique identifier for a recurring transfer.
    pub recurring_transfer_id: String,
    ///The schedule that the recurring transfer will be executed on.
    pub schedule: TransferRecurringSchedule,
    /**The status of the recurring transfer.

`active`: The recurring transfer is currently active.
`cancelled`: The recurring transfer was cancelled by the client or Plaid.
`expired`: The recurring transfer has completed all originations according to its recurring schedule.*/
    pub status: String,
    ///Plaid’s unique identifier for a test clock.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_clock_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transfer_ids: Vec<String>,
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    #[serde(rename = "type")]
    pub type_: String,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for RecurringTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}