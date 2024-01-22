use serde::{Serialize, Deserialize};
use super::RecurringFrequency;
/**Insights relating to expenses and deposits that are predicted to occur on a scheduled basis, such as biweekly, monthly, or annually.

Common examples include loan payments, bill payments, subscriptions, and payroll income.

This is a beta field, available to all users.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Recurrence {
    /**Describes the frequency of the transaction stream.

`WEEKLY`: Assigned to a transaction stream that occurs approximately every week.

`BIWEEKLY`: Assigned to a transaction stream that occurs approximately every 2 weeks.

`SEMI_MONTHLY`: Assigned to a transaction stream that occurs approximately twice per month. This frequency is typically seen for inflow transaction streams.

`MONTHLY`: Assigned to a transaction stream that occurs approximately every month.

`ANNUALLY`: Assigned to a transaction stream that occurs approximately every year.

`DAILY`: Assigned to a transaction stream that occurs approximately every day.

`DYNAMIC`: Assigned to a transaction stream that varies in recurrence frequency. This frequency is typically seen for inflow streams in the gig economy.

`UNKNOWN`: Assigned to a transaction stream that isn't recurring in nature.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RecurringFrequency>,
    ///Whether or not the transaction is periodically recurring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
}
impl std::fmt::Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}