
use serde::{Serialize, Deserialize};
use super::RecurringInsightsStream;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringTransactions {
    pub inflow_streams: Vec<RecurringInsightsStream>,
    pub outflow_streams: Vec<RecurringInsightsStream>,
}
impl std::fmt::Display for RecurringTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}