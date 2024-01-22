use serde::{Serialize, Deserialize};
use super::{RemovedTransaction, Transaction};
///ProcessorTransactionsSyncResponse defines the response schema for `/processor/transactions/sync`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTransactionsSyncResponse {
    ///Transactions that have been added to the Item since `cursor` ordered by ascending last modified time.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<Transaction>,
    ///Represents if more than requested count of transaction updates exist. If true, the additional updates can be fetched by making an additional request with `cursor` set to `next_cursor`. If `has_more` is true, it’s important to pull all available pages, to make it less likely for underlying data changes to conflict with pagination.
    pub has_more: bool,
    ///Transactions that have been modified on the Item since `cursor` ordered by ascending last modified time.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<Transaction>,
    ///Cursor used for fetching any future updates after the latest update provided in this response. The cursor obtained after all pages have been pulled (indicated by `has_more` being `false`) will be valid for at least 1 year. This cursor should be persisted for later calls. If transactions are not yet available, this will be an empty string.
    pub next_cursor: String,
    ///Transactions that have been removed from the Item since `cursor` ordered by ascending last modified time.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<RemovedTransaction>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorTransactionsSyncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}