use serde::{Serialize, Deserialize};
use super::{DistributionBreakdown, PayPeriodDetailsPayFrequency};
///Details about the pay period.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayPeriodDetails {
    ///The amount of the paycheck.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution_breakdown: Option<Vec<DistributionBreakdown>>,
    ///The pay period end date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: "yyyy-mm-dd".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///Total earnings before tax/deductions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_earnings: Option<f64>,
    ///The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<chrono::NaiveDate>,
    ///The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_day: Option<chrono::NaiveDate>,
    ///The frequency at which an individual is paid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<PayPeriodDetailsPayFrequency>,
    ///The pay period start date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: "yyyy-mm-dd".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for PayPeriodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}