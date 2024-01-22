use serde::{Serialize, Deserialize};
///An optional object to be used with the request. If specified, `options` must not be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsSyncRequestOptions {
    /**This option only applies to calls for Items that were not initialized with Transactions during Link and are now adding the Transactions product to the Item for the first time. In these cases, this option controls the maximum number of days of transaction history that Plaid will request from the financial institution. For developer accounts created after December 3, 2023, if no value is specified, this will default to 90 days. For developer accounts created on December 3, 2023 or earlier, if no value is specified, this will default to 730 days until June 24, 2024, at which point it will default to 90 days.

If Transactions has already been added to the Item prior to this call, this field will have no effect.

We strongly recommend that customers utilizing [Recurring Transactions](https://plaid.com/docs/api/products/transactions/#transactionsrecurringget) request at least 180 days of history for optimal results.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
    ///Counterparties and extra merchant fields are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_logo_and_counterparty_beta: Option<bool>,
    ///Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager or submit a [Support request](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/product-functionality).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_original_description: Option<bool>,
    ///Personal finance categories are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category: Option<bool>,
}
impl std::fmt::Display for TransactionsSyncRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}