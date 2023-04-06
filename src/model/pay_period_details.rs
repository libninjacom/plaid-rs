
use serde::{Serialize, Deserialize};
use super::DistributionBreakdown;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayPeriodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_breakdown: Option<Vec<DistributionBreakdown>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_earnings: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_day: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for PayPeriodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}