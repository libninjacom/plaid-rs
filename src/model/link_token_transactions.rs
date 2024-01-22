use serde::{Serialize, Deserialize};
///Configuration parameters for the Transactions product
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenTransactions {
    /**The maximum number of days of transaction history to request for the Transactions product. For developer accounts created after December 3, 2023, if no value is specified, this will default to 90 days. For developer accounts created on December 3, 2023 or earlier, if no value is specified, this will default to 730 days until June 24, 2024, at which point it will default to 90 days.

We strongly recommend that customers utilizing [Recurring Transactions](https://plaid.com/docs/api/products/transactions/#transactionsrecurringget) request at least 180 days of history for optimal results.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
}
impl std::fmt::Display for LinkTokenTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}