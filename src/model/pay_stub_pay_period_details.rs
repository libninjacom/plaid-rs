
use serde::{Serialize, Deserialize};
use super::PayStubDistributionBreakdown;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubPayPeriodDetails {
    pub distribution_breakdown: Vec<PayStubDistributionBreakdown>,
    pub end_date: Option<String>,
    pub gross_earnings: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub pay_amount: Option<f64>,
    pub pay_basis: Option<String>,
    pub pay_date: Option<String>,
    pub pay_frequency: Option<String>,
    pub start_date: Option<String>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for PayStubPayPeriodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}