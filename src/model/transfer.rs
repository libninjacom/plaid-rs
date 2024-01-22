use serde::{Serialize, Deserialize};
use super::{
    TransferAuthorizationGuaranteeDecision,
    TransferAuthorizationGuaranteeDecisionRationale, TransferCreditFundsSource,
    TransferExpectedSweepSettlementScheduleItem, TransferFailure, TransferMetadata,
    TransferRefund, TransferSweepStatus, TransferUserInResponse,
};
///Represents a transfer within the Transfers API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transfer {
    ///The Plaid `account_id` corresponding to the end-user account that will be debited or credited.
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
    ///Plaid’s unique identifier for a transfer authorization.
    pub authorization_id: String,
    ///When `true`, you can still cancel this transfer.
    pub cancellable: bool,
    ///The datetime when this transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: chrono::DateTime<chrono::Utc>,
    pub credit_funds_source: TransferCreditFundsSource,
    ///The description of the transfer.
    pub description: String,
    ///The expected date when the full amount of the transfer settles at the consumers’ account, if the transfer is credit; or at the customer's business checking account, if the transfer is debit. Only set for ACH transfers and is null for non-ACH transfers. Only set for ACH transfers. This will be of the form YYYY-MM-DD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_settlement_date: Option<chrono::NaiveDate>,
    ///The expected sweep settlement schedule of this transfer, assuming this transfer is not `returned`. Only applies to ACH debit transfers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_sweep_settlement_schedule: Option<
        Vec<TransferExpectedSweepSettlementScheduleItem>,
    >,
    ///The amount to deduct from `transfer.amount` and distribute to the platform’s Ledger balance as a facilitator fee (decimal string with two digits of precision e.g. "10.00"). The remainder will go to the end-customer’s Ledger balance. This must be less than or equal to the `transfer.amount`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facilitator_fee: Option<String>,
    ///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TransferFailure>,
    ///The id of the associated funding account, available in the Plaid Dashboard. If present, this indicates which of your business checking accounts will be credited or debited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<String>,
    ///Indicates whether the transfer is guaranteed by Plaid (Guarantee customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    ///Plaid’s unique identifier for a transfer.
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
    /**The network or rails used for the transfer.

For transfers submitted as `ach`, the next-day cutoff is 5:30 PM Eastern Time.

For transfers submitted as `same-day-ach`, the same-day cutoff is 3:30 PM Eastern Time. If the transfer is submitted after this cutoff but before the next-day cutoff, it will be sent over next-day rails and will not incur same-day charges; this will apply to both legs of the transfer if applicable.

For transfers submitted as `rtp`,  Plaid will automatically route between Real Time Payment rail by TCH or FedNow rails as necessary. If a transfer is submitted as `rtp` and the counterparty account is not eligible for RTP, the `/transfer/authorization/create` request will fail with an `INVALID_FIELD` error code. To pre-check to determine whether a counterparty account can support RTP, call `/transfer/capabilities/get` before calling `/transfer/authorization/create`.*/
    pub network: String,
    /**The trace identifier for the transfer based on its network. This will only be set after the transfer has posted.

For `ach` or `same-day-ach` transfers, this is the ACH trace number. Currently, the field will remain null for transfers on other rails.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_trace_id: Option<String>,
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///The Plaid client ID that is the originator of this transfer. Only present if created on behalf of another client as a [Platform customer](https://plaid.com/docs/transfer/application/#originators-vs-platforms).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<String>,
    ///The id of the recurring transfer if this transfer belongs to a recurring transfer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_transfer_id: Option<String>,
    ///A list of refunds associated with this transfer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refunds: Vec<TransferRefund>,
    ///The date 3 business days from settlement date indicating the following ACH returns can no longer happen: R01, R02, R03, R29. This will be of the form YYYY-MM-DD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_return_window: Option<chrono::NaiveDate>,
    /**The status of the transfer.

`pending`: A new transfer was created; it is in the pending state.
`posted`: The transfer has been successfully submitted to the payment network.
`settled`: Credits are available to be withdrawn or debits have been deducted from the Plaid linked account.
`cancelled`: The transfer was cancelled by the client.
`failed`: The transfer failed, no funds were moved.
`returned`: A posted transfer was returned.*/
    pub status: String,
    /**The status of the sweep for the transfer.

`unswept`: The transfer hasn't been swept yet.
`swept`: The transfer was swept to the sweep account.
`swept_settled`: Credits are available to be withdrawn or debits have been deducted from the customer’s business checking account.
`return_swept`: The transfer was returned, funds were pulled back or pushed back to the sweep account.
`null`: The transfer will never be swept (e.g. if the transfer is cancelled or returned before being swept)*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sweep_status: Option<TransferSweepStatus>,
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    #[serde(rename = "type")]
    pub type_: String,
    ///The date 61 business days from settlement date indicating the following ACH returns can no longer happen: R05, R07, R10, R11, R51, R33, R37, R38, R51, R52, R53. This will be of the form YYYY-MM-DD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_return_window: Option<chrono::NaiveDate>,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}