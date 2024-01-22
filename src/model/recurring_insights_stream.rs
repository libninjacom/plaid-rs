use serde::{Serialize, Deserialize};
use super::TransactionStreamAmount;
///Insights object for recurring transactions streams.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringInsightsStream {
    ///Object with data pertaining to an amount on the transaction stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_amount: Option<TransactionStreamAmount>,
    ///The average number of days between each of the recurring transactions.
    pub average_days_apart: f64,
    ///The client-provided raw description of the most recent transaction in the stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /**Describes the frequency of the transaction stream.

`WEEKLY`: Assigned to a transaction stream that occurs approximately every week.

`BIWEEKLY`: Assigned to a transaction stream that occurs approximately every 2 weeks.

`SEMI_MONTHLY`: Assigned to a transaction stream that occurs approximately twice per month. This frequency is typically seen for inflow transaction streams.

`MONTHLY`: Assigned to a transaction stream that occurs approximately every month.

`ANNUALLY`: Assigned to a transaction stream that occurs approximately every year.

`UNKNOWN`: Assigned to a transaction stream that does not fit any of the pre-defined frequencies.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    ///Indicates whether the transaction stream is still live.
    pub is_active: bool,
    ///The merchant or primary counterparty associated with the transaction stream.
    pub merchant_name: String,
    ///Object with data pertaining to an amount on the transaction stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub newest_transaction_amount: Option<TransactionStreamAmount>,
    ///The posted date of the latest transaction in the stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub newest_transaction_date: Option<chrono::NaiveDate>,
    ///The posted date of the earliest transaction in the stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<chrono::NaiveDate>,
    ///The detailed category associated with the transaction stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_detailed: Option<String>,
    ///The primary category associated with the transaction stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_primary: Option<String>,
    /**The current status of the transaction stream.

`MATURE`: A `MATURE` recurring stream should have at least 3 transactions and happen on a regular cadence (For Annual recurring stream, we will mark it `MATURE` after 2 instances).

`EARLY_DETECTION`: When a recurring transaction first appears in the transaction history and before it fulfills the requirement of a mature stream, the status will be `EARLY_DETECTION`.

`TOMBSTONED`: A stream that was previously in the `EARLY_DETECTION` status will move to the `TOMBSTONED` status when no further transactions were found at the next expected date.

`UNKNOWN`: A stream is assigned an `UNKNOWN` status when none of the other statuses are applicable.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///A unique id for the stream.
    pub stream_id: String,
    ///The number of transactions in this stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i64>,
    ///An array of Plaid transaction IDs belonging to the stream, sorted by posted date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_ids: Option<Vec<String>>,
}
impl std::fmt::Display for RecurringInsightsStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}