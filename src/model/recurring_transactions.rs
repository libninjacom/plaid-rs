use serde::{Serialize, Deserialize};
use super::RecurringInsightsStream;
///Insights object for recurring transactions for `/beta/transactions/user_insights/v1/get` endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringTransactions {
    ///An array of inflow transaction streams (e.g., income).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inflow_streams: Vec<RecurringInsightsStream>,
    ///An array of outflow transaction streams (e.g., subscriptions, bills, loan payments).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outflow_streams: Vec<RecurringInsightsStream>,
}
impl std::fmt::Display for RecurringTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}