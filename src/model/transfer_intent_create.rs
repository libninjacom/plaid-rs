use serde::{Serialize, Deserialize};
use super::{TransferMetadata, TransferUserInResponse};
///Represents a transfer intent within Transfer UI.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferIntentCreate {
    ///The Plaid `account_id` corresponding to the end-user account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, e.g. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<String>,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00"). When calling `/transfer/authorization/create`, specify the maximum amount to authorize. When calling `/transfer/create`, specify the exact amount of the transfer, up to a maximum of the amount authorized. If this field is left blank when calling `/transfer/create`, the maximum amount authorized in the `authorization_id` will be sent.
    pub amount: String,
    ///The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    pub created: chrono::DateTime<chrono::Utc>,
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    ///The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited.
    pub funding_account_id: String,
    ///Plaid's unique identifier for the transfer intent object.
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
    pub metadata: Option<TransferMetadata>,
    /**The direction of the flow of transfer funds.

`PAYMENT`: Transfers funds from an end user's account to your business account.

`DISBURSEMENT`: Transfers funds from your business account to an end user's account.*/
    pub mode: String,
    /**The network or rails used for the transfer. Defaults to `same-day-ach`.

For transfers submitted as `ach`, the next-day cutoff is 5:30 PM Eastern Time.

For transfers submitted as `same-day-ach`, the same-day cutoff is 3:30 PM Eastern Time. If the transfer is submitted after this cutoff but before the next-day cutoff, it will be sent over next-day rails and will not incur same-day charges; this will apply to both legs of the transfer if applicable.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    ///Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    pub origination_account_id: String,
    ///When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guarantee customers only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_guarantee: Option<bool>,
    /**The status of the transfer intent.

`PENDING`: The transfer intent is pending.
`SUCCEEDED`: The transfer intent was successfully created.
`FAILED`: The transfer intent was unable to be created.*/
    pub status: String,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for TransferIntentCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}