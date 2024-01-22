use serde::{Serialize, Deserialize};
///An optional object to be used with the request. If specified, `options` must not be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTransactionsGetRequestOptions {
    ///The number of transactions to fetch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    ///Counterparties and extra merchant fields are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_logo_and_counterparty_beta: Option<bool>,
    ///Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager, or submit a [Support request](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/product-functionality).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_original_description: Option<bool>,
    ///Personal finance categories are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category: Option<bool>,
    ///Personal finance categories are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category_beta: Option<bool>,
    ///The number of transactions to skip. The default value is 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
impl std::fmt::Display for ProcessorTransactionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}