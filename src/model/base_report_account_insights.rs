
use serde::{Serialize, Deserialize};
use super::{
    BaseReportAverageFlowInsights, BaseReportLongestGapInsights,
    BaseReportNumberFlowInsights,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAccountInsights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_days_between_transactions: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_inflow_amount: Option<Vec<BaseReportAverageFlowInsights>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_outflow_amount: Option<Vec<BaseReportAverageFlowInsights>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_available: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longest_gap_between_transactions: Option<Vec<BaseReportLongestGapInsights>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_transaction_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_days_no_transactions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_inflows: Option<Vec<BaseReportNumberFlowInsights>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_outflows: Option<Vec<BaseReportNumberFlowInsights>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for BaseReportAccountInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}