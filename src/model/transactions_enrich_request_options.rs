use serde::{Serialize, Deserialize};
///An optional object to be used with the request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnrichRequestOptions {
    /**Include `legacy_category` and `legacy_category_id` in the response (in addition to the default `personal_finance_category`).

Categories are based on Plaid's legacy taxonomy. For a full list of legacy categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_legacy_category: Option<bool>,
}
impl std::fmt::Display for TransactionsEnrichRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}