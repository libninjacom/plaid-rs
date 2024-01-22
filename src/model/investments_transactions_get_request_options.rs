use serde::{Serialize, Deserialize};
///An optional object to filter `/investments/transactions/get` results. If provided, must be non-`null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsTransactionsGetRequestOptions {
    ///An array of `account_ids` to retrieve for the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    ///If the Item was not initialized with the investments product via the `products` array when calling `/link/token/create`, and `async_update` is set to true, the initial Investments extraction will happen asynchronously. Plaid will subsequently fire a `HISTORICAL_UPDATE` webhook when the extraction completes. When `false`, Plaid will wait to return a response until extraction completion and no `HISTORICAL_UPDATE` webhook will fire. Note that while the extraction is happening asynchronously, calls to `/investments/transactions/get` and `/investments/refresh` will return `PRODUCT_NOT_READY` errors until the extraction completes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub async_update: Option<bool>,
    ///The number of transactions to fetch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    ///The number of transactions to skip when fetching transaction history
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
impl std::fmt::Display for InvestmentsTransactionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}