
use serde::{Serialize, Deserialize};
use super::{
    CategoryInsights, CounterpartyInsights, RecurringTransactions, UserDataOverview,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsUserInsightsGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_insights: Option<CategoryInsights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_insights: Option<CounterpartyInsights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_transactions: Option<RecurringTransactions>,
    pub user_data_overview: UserDataOverview,
}
impl std::fmt::Display for TransactionsUserInsightsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}