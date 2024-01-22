use serde::{Serialize, Deserialize};
use super::{PersonalFinanceCategory, TransactionStreamAmount};
///A grouping of related transactions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionStream {
    ///The ID of the account to which the stream belongs
    pub account_id: String,
    ///Object with data pertaining to an amount on the transaction stream.
    pub average_amount: TransactionStreamAmount,
    /**A hierarchical array of the categories to which this transaction belongs. See [Categories](https://plaid.com/docs/api/products/transactions/#categoriesget).

All implementations are encouraged to use the new `personal_finance_category` instead of `category`. `personal_finance_category` provides more meaningful categorization and greater accuracy.*/
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<String>,
    /**The ID of the category to which this transaction belongs. See [Categories](https://plaid.com/docs/api/products/transactions/#categoriesget).

All implementations are encouraged to use the new `personal_finance_category` instead of `category`. `personal_finance_category` provides more meaningful categorization and greater accuracy.*/
    pub category_id: String,
    ///A description of the transaction stream.
    pub description: String,
    ///The posted date of the earliest transaction in the stream.
    pub first_date: chrono::NaiveDate,
    /**Describes the frequency of the transaction stream.

`WEEKLY`: Assigned to a transaction stream that occurs approximately every week.

`BIWEEKLY`: Assigned to a transaction stream that occurs approximately every 2 weeks.

`SEMI_MONTHLY`: Assigned to a transaction stream that occurs approximately twice per month. This frequency is typically seen for inflow transaction streams.

`MONTHLY`: Assigned to a transaction stream that occurs approximately every month.

`ANNUALLY`: Assigned to a transaction stream that occurs approximately every year.

`UNKNOWN`: Assigned to a transaction stream that does not fit any of the pre-defined frequencies.*/
    pub frequency: String,
    ///Indicates whether the transaction stream is still live.
    pub is_active: bool,
    ///This will be set to `true` if the stream has been modified by request to a `/transactions/recurring/streams` endpoint. It will be `false` for all other streams.
    pub is_user_modified: bool,
    ///Object with data pertaining to an amount on the transaction stream.
    pub last_amount: TransactionStreamAmount,
    ///The posted date of the latest transaction in the stream.
    pub last_date: chrono::NaiveDate,
    ///The date and time of the most recent user modification. This will only be set if `is_user_modified` is `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_user_modified_datetime: Option<chrono::DateTime<chrono::Utc>>,
    ///The merchant associated with the transaction stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /**Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.

See the [`taxonomy CSV file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories. If you are migrating to personal finance categories from the legacy categories, also refer to the [`migration guide`](https://plaid.com/docs/transactions/pfc-migration/).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    /**The current status of the transaction stream.

`MATURE`: A `MATURE` recurring stream should have at least 3 transactions and happen on a regular cadence (For Annual recurring stream, we will mark it `MATURE` after 2 instances).

`EARLY_DETECTION`: When a recurring transaction first appears in the transaction history and before it fulfills the requirement of a mature stream, the status will be `EARLY_DETECTION`.

`TOMBSTONED`: A stream that was previously in the `EARLY_DETECTION` status will move to the `TOMBSTONED` status when no further transactions were found at the next expected date.

`UNKNOWN`: A stream is assigned an `UNKNOWN` status when none of the other statuses are applicable.*/
    pub status: String,
    ///A unique id for the stream
    pub stream_id: String,
    ///An array of Plaid transaction IDs belonging to the stream, sorted by posted date.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transaction_ids: Vec<String>,
}
impl std::fmt::Display for TransactionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}