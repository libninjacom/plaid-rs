use serde::{Serialize, Deserialize};
use super::{
    ExternalPaymentRefundDetails, ExternalPaymentScheduleBase, PaymentAmount,
    PaymentAmountRefunded, PaymentScheme, RecipientBacs,
};
///PaymentInitiationPayment defines a payment initiation payment
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPayment {
    ///The value of the reference sent to the bank after adjustment to pass bank validation rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjusted_reference: Option<String>,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`LOCAL_DEFAULT`: The default payment scheme for the selected market and currency will be used.

`LOCAL_INSTANT`: The instant payment scheme for the selected market and currency will be used (if applicable). Fees may be applied by the institution. If the market does not support an Instant Scheme (e.g. Denmark), the default in the region will be used.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjusted_scheme: Option<PaymentScheme>,
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    ///The amount and currency of a payment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<PaymentAmountRefunded>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    ///The payment consent ID that this payment was initiated with. Is present only when payment was initiated using the payment consent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,
    ///The International Bank Account Number (IBAN) for the sender, if specified in the `/payment_initiation/payment/create` call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    ///The date and time of the last time the `status` was updated, in IS0 8601 format
    pub last_status_update: chrono::DateTime<chrono::Utc>,
    ///The ID of the payment. Like all Plaid identifiers, the `payment_id` is case sensitive.
    pub payment_id: String,
    ///The ID of the recipient
    pub recipient_id: String,
    ///A reference for the payment.
    pub reference: String,
    ///Details about external payment refund
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_details: Option<ExternalPaymentRefundDetails>,
    ///Refund IDs associated with the payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_ids: Option<Vec<String>>,
    ///The schedule that the payment will be executed on. If a schedule is provided, the payment is automatically set up as a standing order. If no schedule is specified, the payment will be executed only once.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ExternalPaymentScheduleBase>,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`LOCAL_DEFAULT`: The default payment scheme for the selected market and currency will be used.

`LOCAL_INSTANT`: The instant payment scheme for the selected market and currency will be used (if applicable). Fees may be applied by the institution. If the market does not support an Instant Scheme (e.g. Denmark), the default in the region will be used.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<PaymentScheme>,
    /**The status of the payment.

`PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.

`PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution. For successful payments, this is a potential terminal status. Further status transitions can be to REJECTED and, when supported by the institution, to EXECUTED.

`PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.

`PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error may be caused by transient system outages and is retryable once the root cause is resolved.

`PAYMENT_STATUS_BLOCKED`: The payment has been blocked by Plaid. This can occur, for example, due to Plaid flagging the payment as potentially risky. This is a retryable error.

`PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.

`PAYMENT_STATUS_CANCELLED`: The payment was cancelled (typically by the end user) during authorisation.

`PAYMENT_STATUS_EXECUTED`: The funds have successfully left the payer account and payment is considered complete. Not all institutions support this status: support is more common in the UK, and less common in the EU. For institutions where this status is not supported, the terminal status for a successful payment will be `PAYMENT_STATUS_INITIATED`.

`PAYMENT_STATUS_SETTLED`: The payment has settled and funds are available for use. A payment will typically settle within seconds to several days, depending on which payment rail is used. This status is only available to customers using [Plaid Virtual Accounts](https://plaid.com/docs/virtual-accounts/).

`PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

`PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.

Deprecated:
These statuses will be removed in a future release.

`PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.

`PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.

`PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.*/
    pub status: String,
    ///The transaction ID that this payment is associated with, if any. This is present only when a payment was initiated using virtual accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    ///The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}
impl std::fmt::Display for PaymentInitiationPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}