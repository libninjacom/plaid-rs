
use serde::{Serialize, Deserialize};
use super::DistributionBreakdown;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayPeriodDetails {
    pub check_amount: Option<f64>,
    pub distribution_breakdown: Option<Vec<DistributionBreakdown>>,
    pub end_date: Option<String>,
    pub gross_earnings: Option<f64>,
    pub pay_date: Option<String>,
    pub pay_day: Option<String>,
    pub pay_frequency: Option<serde_json::Value>,
    pub start_date: Option<String>,
}
impl std::fmt::Display for PayPeriodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}