use serde::{Serialize, Deserialize};
use super::{
    CategoryInsights, CounterpartyInsights, RecurringTransactions, UserDataOverview,
};
///TransactionsUserInsightsGetResponse defines the response schema for `/beta/transactions/user_insights/v1/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsUserInsightsGetResponse {
    ///Insights on a user's top personal finance categories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_insights: Option<CategoryInsights>,
    ///Insights around a user's counterparties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counterparty_insights: Option<CounterpartyInsights>,
    ///Insights object for recurring transactions for `/beta/transactions/user_insights/v1/get` endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_transactions: Option<RecurringTransactions>,
    ///metadata for the set of insights provided in `TransactionsUserInsightsGetResponse`
    pub user_data_overview: UserDataOverview,
}
impl std::fmt::Display for TransactionsUserInsightsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}