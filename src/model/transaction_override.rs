use serde::{Serialize, Deserialize};
///Data to populate as test transaction data. If not specified, random transactions will be generated instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionOverride {
    ///The transaction amount. Can be negative.
    pub amount: f64,
    ///The ISO-4217 format currency code for the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///The date the transaction posted, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Posted dates in the past or present will result in posted transactions; posted dates in the future will result in pending transactions.
    pub date_posted: chrono::NaiveDate,
    ///The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Transactions in Sandbox will move from pending to posted once their transaction date has been reached. If a `date_transacted` is not provided by the institution, a transaction date may be available in the [`authorized_date`](https://plaid.com/docs/api/products/transactions/#transactions-get-response-transactions-authorized-date) field.
    pub date_transacted: chrono::NaiveDate,
    ///The transaction description.
    pub description: String,
}
impl std::fmt::Display for TransactionOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}