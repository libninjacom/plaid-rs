
use serde::{Serialize, Deserialize};
use super::CreditAmountWithCurrency;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAverageFlowInsights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<CreditAmountWithCurrency>,
}
impl std::fmt::Display for BaseReportAverageFlowInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}