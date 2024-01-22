use serde::{Serialize, Deserialize};
///metadata for the set of insights provided in `TransactionsUserInsightsGetResponse`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserDataOverview {
    ///The range of days of transactions available.
    pub days_available: i64,
    ///The date of the newest transaction processed to generate insights.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub newest_transaction_date: Option<chrono::NaiveDate>,
    ///The date of the oldest transaction processed to generate insights.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<chrono::NaiveDate>,
    ///Sum of inflow amounts.
    pub total_inflows: f64,
    ///Sum of outflow amounts.
    pub total_outflows: f64,
    ///The total number of transactions.
    pub transaction_count: i64,
}
impl std::fmt::Display for UserDataOverview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}