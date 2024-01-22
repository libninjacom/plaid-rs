use serde::{Serialize, Deserialize};
use super::{CreditCategory, Location, PaymentMeta};
///A transaction on the asset report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportTransaction {
    ///The ID of the account in which this transaction occurred.
    pub account_id: String,
    ///The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    ///The settled value of the transaction, denominated in the transaction's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    /**A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).

This field will only appear in an Asset Report with Insights.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    /**The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).

This field will only appear in an Asset Report with Insights.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    ///The check number of the transaction. This field is only populated for check transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /**Information describing the intent of the transaction. Most relevant for credit use cases, but not limited to such use cases.

See the [`taxonomy csv file`](https://plaid.com/documents/credit-category-taxonomy.csv) for a full list of credit categories.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_category: Option<CreditCategory>,
    ///For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    pub date: chrono::NaiveDate,
    ///The date on which the transaction took place, in IS0 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_transacted: Option<String>,
    ///A unique identifier for an income source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_source_id: Option<String>,
    ///The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    ///A representation of where a transaction took place
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    ///The merchant name, as enriched by Plaid from the `name` field. This is typically a more human-readable version of the merchant counterparty in the transaction. For some bank transactions (such as checks or account transfers) where there is no meaningful merchant name, this value will be `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /**The merchant name or transaction description.

This field will only appear in an Asset Report with Insights.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The string returned by the financial institution to describe the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_description: Option<String>,
    /**Transaction information specific to inter-bank transfers. If the transaction was not an inter-bank transfer, all fields will be `null`.

If the `transactions` object was returned by a Transactions endpoint such as `/transactions/sync` or `/transactions/get`, the `payment_meta` key will always appear, but no data elements are guaranteed. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_meta: Option<PaymentMeta>,
    ///When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    pub pending: bool,
    ///The ID of a posted transaction's associated pending transaction, where applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_transaction_id: Option<String>,
    ///The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    pub transaction_id: String,
    /**`digital:` transactions that took place online.

`place:` transactions that were made at a physical location.

`special:` transactions that relate to banks, e.g. fees or deposits.

`unresolved:` transactions that do not fit into the other three types.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    /**The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `unofficial_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for AssetReportTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}