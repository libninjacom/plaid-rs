
use serde::{Serialize, Deserialize};
use super::PayStubDistributionBreakdown;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubPayPeriodDetails {
    pub distribution_breakdown: Vec<PayStubDistributionBreakdown>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_earnings: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_basis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for PayStubPayPeriodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}