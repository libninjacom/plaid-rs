use serde::{Serialize, Deserialize};
///Fired when the status of a wallet transaction has changed.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionStatusUpdateWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    /**The status of the transaction.

`AUTHORISING`: The transaction is being processed for validation and compliance.

`INITIATED`: The transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed and is considered complete. This is only applicable for debit transactions.

`SETTLED`: The transaction has settled and funds are available for use. This is only applicable for credit transactions. A transaction will typically settle within seconds to several days, depending on which payment rail is used.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub new_status: String,
    /**The status of the transaction.

`AUTHORISING`: The transaction is being processed for validation and compliance.

`INITIATED`: The transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed and is considered complete. This is only applicable for debit transactions.

`SETTLED`: The transaction has settled and funds are available for use. This is only applicable for credit transactions. A transaction will typically settle within seconds to several days, depending on which payment rail is used.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub old_status: String,
    ///The `payment_id` associated with the transaction. This will be present in case of `REFUND` and `PIS_PAY_IN`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    ///The timestamp of the update, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2017-09-14T14:42:19.350Z"`
    pub timestamp: chrono::DateTime<chrono::Utc>,
    ///The `transaction_id` for the wallet transaction being updated
    pub transaction_id: String,
    ///The EMI (E-Money Institution) wallet that this payment is associated with. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    ///`WALLET_TRANSACTION_STATUS_UPDATE`
    pub webhook_code: String,
    ///`WALLET`
    pub webhook_type: String,
}
impl std::fmt::Display for WalletTransactionStatusUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}