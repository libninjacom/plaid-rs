use serde::{Serialize, Deserialize};
use super::{WalletPaymentScheme, WalletTransactionAmount, WalletTransactionCounterparty};
///The transaction details
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransaction {
    ///The amount and currency of a transaction
    pub amount: WalletTransactionAmount,
    ///An object representing the e-wallet transaction's counterparty
    pub counterparty: WalletTransactionCounterparty,
    ///Timestamp when the transaction was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**The error code of a failed transaction. Error codes include:
`EXTERNAL_SYSTEM`: The transaction was declined by an external system.
`EXPIRED`: The transaction request has expired.
`CANCELLED`: The transaction request was rescinded.
`INVALID`: The transaction did not meet certain criteria, such as an inactive account or no valid counterparty, etc.
`UNKNOWN`: The transaction was unsuccessful, but the exact cause is unknown.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    ///The date and time of the last time the `status` was updated, in IS0 8601 format
    pub last_status_update: chrono::DateTime<chrono::Utc>,
    ///The payment id that this transaction is associated with, if any. This is present only for transaction types `PIS_PAY_IN` and `REFUND`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    ///A reference for the transaction
    pub reference: String,
    /**The payment scheme used to execute this transaction. This is present only for transaction types `PAYOUT` and `REFUND`.

`FASTER_PAYMENTS`: The standard payment scheme within the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment to a beneficiary within the SEPA area.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<WalletPaymentScheme>,
    /**The status of the transaction.

`AUTHORISING`: The transaction is being processed for validation and compliance.

`INITIATED`: The transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed and is considered complete. This is only applicable for debit transactions.

`SETTLED`: The transaction has settled and funds are available for use. This is only applicable for credit transactions. A transaction will typically settle within seconds to several days, depending on which payment rail is used.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub status: String,
    ///A unique ID identifying the transaction
    pub transaction_id: String,
    /**The type of the transaction. The supported transaction types that are returned are:
`BANK_TRANSFER:` a transaction which credits an e-wallet through an external bank transfer.

`PAYOUT:` a transaction which debits an e-wallet by disbursing funds to a counterparty.

`PIS_PAY_IN:` a payment which credits an e-wallet through Plaid's Payment Initiation Services (PIS) APIs. For more information see the [Payment Initiation endpoints](https://plaid.com/docs/api/products/payment-initiation/).

`REFUND:` a transaction which debits an e-wallet by refunding a previously initiated payment made through Plaid's [PIS APIs](https://plaid.com/docs/api/products/payment-initiation/).

`FUNDS_SWEEP`: an automated transaction which debits funds from an e-wallet to a designated client-owned account.

`RETURN`: an automated transaction where a debit transaction was reversed and money moved back to originating account.*/
    #[serde(rename = "type")]
    pub type_: String,
    ///The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: String,
}
impl std::fmt::Display for WalletTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}