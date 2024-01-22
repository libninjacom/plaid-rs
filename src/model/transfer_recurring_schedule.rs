use serde::{Serialize, Deserialize};
///The schedule that the recurring transfer will be executed on.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringSchedule {
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). The recurring transfer will end on the last `interval_execution_day` on or before the `end_date`.
If the `interval_execution_day` between the start date and the end date (inclusive) is also the same day that `/transfer/recurring/create` was called, the bank *may* make a payment on that day, but it is not guaranteed to do so.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    /**The number of recurring `interval_units` between originations. The recurring interval (before holiday adjustment) is calculated by multiplying `interval_unit` and `interval_count`.
For example, to schedule a recurring transfer which originates once every two weeks, set `interval_unit` = `week` and `interval_count` = 2.*/
    pub interval_count: i64,
    /**The day of the interval on which to schedule the transfer.

If the `interval_unit` is `week`, `interval_execution_day` should be an integer from 1 (Monday) to 5 (Friday).

If the `interval_unit` is `month`, `interval_execution_day` should be an integer indicating which day of the month to make the transfer on. Integers from 1 to 28 can be used to make a transfer on that day of the month. Negative integers from -1 to -5 can be used to make a transfer relative to the end of the month. To make a transfer on the last day of the month, use -1; to make the transfer on the second-to-last day, use -2, and so on.

The transfer will be originated on the next available banking day if the designated day is a non banking day.*/
    pub interval_execution_day: i64,
    ///The unit of the recurring interval.
    pub interval_unit: String,
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). The recurring transfer will begin on the first `interval_execution_day` on or after the `start_date`.

If the first `interval_execution_day` on or after the start date is also the same day that `/transfer/recurring/create` was called, the bank *may* make the first payment on that day, but it is not guaranteed to do so.*/
    pub start_date: chrono::NaiveDate,
}
impl std::fmt::Display for TransferRecurringSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}