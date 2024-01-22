use serde::{Serialize, Deserialize};
use super::{AccountBase, Transaction};
///ProcessorTransactionsGetResponse defines the response schema for `/processor/transactions/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTransactionsGetResponse {
    ///A single account at a financial institution.
    pub account: AccountBase,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///The total number of transactions available within the date range specified. If `total_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.
    pub total_transactions: i64,
    ///An array containing transactions from the account. Transactions are returned in reverse chronological order, with the most recent at the beginning of the array. The maximum number of transactions returned is determined by the `count` parameter.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<Transaction>,
}
impl std::fmt::Display for ProcessorTransactionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}